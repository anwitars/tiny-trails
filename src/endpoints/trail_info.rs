use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};

use crate::response::TTResponse;

use super::{
    common::{SameErrorResult, TRAIL_SECRET_HEADER},
    resolve::TRAIL_NOT_FOUND_OR_EXPIRED_MSG,
};

/// Fields that are returned only if the user is authenticated for the specific trail
/// using their secret
type OnlyWithAuth<T> = Option<T>;
type ResponseUtcTime = String;

macro_rules! if_has_auth {
    ($auth:ident, $value:expr) => {
        if $auth {
            let getter = || Some($value);
            getter()
        } else {
            None
        }
    };
}

#[derive(serde::Deserialize)]
pub struct TrailInfoQueryParams {
    #[serde(default, rename = "week_history")]
    with_week_history: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TrailInfoResponseData {
    pub trailid: String,
    pub long: String,
    pub unique_tracks: i64,
    pub total_tracks: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: OnlyWithAuth<ResponseUtcTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_hours: OnlyWithAuth<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: OnlyWithAuth<ResponseUtcTime>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_history: OnlyWithAuth<Vec<ResponseUtcTime>>,
}

pub enum TrailInfoResponse {
    NotFound,
    TTResponse(TTResponse<TrailInfoResponseData>),
}

impl IntoResponse for TrailInfoResponse {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => Response::builder()
                .status(404)
                .body(TRAIL_NOT_FOUND_OR_EXPIRED_MSG.into())
                .unwrap(),
            Self::TTResponse(response) => response.into_response(),
        }
    }
}

impl From<sqlx::Error> for TrailInfoResponse {
    fn from(err: sqlx::Error) -> Self {
        Self::TTResponse(TTResponse::from(err))
    }
}

pub async fn trail_info(
    State(pool): State<sqlx::PgPool>,
    Path(trailid): Path<String>,
    Query(query_params): Query<TrailInfoQueryParams>,
    headers: HeaderMap,
) -> SameErrorResult<TrailInfoResponse> {
    let trail = sqlx::query!(
        r#"
        SELECT id, long, created_at, expiration_hours, secret
        FROM trails
        WHERE short = $1
        "#,
        trailid
    )
    .fetch_optional(&pool)
    .await?;

    if trail.is_none() {
        return Ok(TrailInfoResponse::NotFound);
    }
    let trail = trail.unwrap();

    let has_auth = headers.get(TRAIL_SECRET_HEADER).map_or(false, |secret| {
        secret
            .to_str()
            .map_or(false, |secret| secret == trail.secret)
    });

    let track_info = sqlx::query!(
        r#"
        SELECT
            COUNT(DISTINCT CASE WHEN hashed_ip IS NULL THEN id::text ELSE hashed_ip END) AS unique_tracks,
            COUNT(id) AS total_tracks
        FROM tracks
        WHERE trail_id = $1
        "#,
        trail.id
    )
    .fetch_one(&pool)
    .await?;

    let created_at = if_has_auth!(has_auth, trail.created_at.to_string());
    let expiration_hours = if_has_auth!(has_auth, trail.expiration_hours);
    let expires_at = if_has_auth!(
        has_auth,
        (trail.created_at + chrono::Duration::hours(trail.expiration_hours as i64)).to_string()
    );

    let week_history = if has_auth && query_params.with_week_history {
        let week_history = sqlx::query!(
            r#"
            SELECT created_at
            FROM tracks
            WHERE trail_id = $1 AND created_at > NOW() - INTERVAL '1 week'
            ORDER BY created_at DESC
            "#,
            trail.id
        )
        .fetch_all(&pool)
        .await?
        .into_iter()
        .map(|row| row.created_at.to_string())
        .collect();

        Some(week_history)
    } else {
        None
    };

    Ok(TrailInfoResponse::TTResponse(TTResponse::Data(
        TrailInfoResponseData {
            trailid,
            long: trail.long,
            unique_tracks: track_info.unique_tracks.unwrap_or(0),
            total_tracks: track_info.total_tracks.unwrap_or(0),

            created_at,
            expiration_hours,
            expires_at,

            week_history,
        },
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{
        app::app,
        utils::testing::{BodyDeserializeJson, BodyToString},
    };

    async fn setup_db(pool: &sqlx::PgPool) {
        let trail_db_id = sqlx::query!(
            r#"
            INSERT INTO trails (short, long, expiration_hours, secret)
            VALUES ('test', 'https://example.com', 1, 'wow')
            RETURNING id
            "#
        )
        .fetch_one(pool)
        .await
        .unwrap()
        .id;

        async fn insert_track(
            pool: &sqlx::PgPool,
            trail_db_id: i32,
            hashed_ip: Option<&str>,
            created_at: Option<&chrono::DateTime<chrono::Utc>>,
        ) {
            let hashed_ip = hashed_ip.map(String::from);
            let created_at = created_at
                .map(|dt| dt.naive_utc())
                .unwrap_or_else(|| chrono::Utc::now().naive_utc());

            sqlx::query!(
                r#"
                INSERT INTO tracks (trail_id, hashed_ip, created_at)
                VALUES ($1, $2, $3)
                "#,
                trail_db_id,
                hashed_ip,
                created_at
            )
            .execute(pool)
            .await
            .unwrap();
        }

        let more_than_one_week_ago = chrono::Utc::now() - chrono::Duration::weeks(2);
        let more_than_one_week_ago = Some(&more_than_one_week_ago);

        insert_track(pool, trail_db_id, Some("one"), None).await;
        insert_track(pool, trail_db_id, Some("two"), None).await;
        insert_track(pool, trail_db_id, Some("two"), None).await;
        insert_track(pool, trail_db_id, Some("three"), None).await;
        insert_track(pool, trail_db_id, Some("three"), None).await;
        insert_track(pool, trail_db_id, Some("three"), more_than_one_week_ago).await;

        insert_track(pool, trail_db_id, None, None).await;
        insert_track(pool, trail_db_id, None, more_than_one_week_ago).await;
        insert_track(pool, trail_db_id, None, more_than_one_week_ago).await;
    }

    #[sqlx::test]
    async fn test_ok_without_secret(pool: sqlx::PgPool) {
        setup_db(&pool).await;

        let app = app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/info/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response
            .into_body()
            .deserialize_json::<TTResponse<TrailInfoResponseData>>()
            .await;

        assert!(matches!(body, TTResponse::Data(_)));
        let body = body.unwrap_data();

        assert_eq!(body.trailid, "test");
        assert_eq!(body.long, "https://example.com");
        assert_eq!(body.unique_tracks, 6);
        assert_eq!(body.total_tracks, 9);

        assert_eq!(body.expiration_hours, None);
        assert_eq!(body.expires_at, None);
        assert_eq!(body.created_at, None);

        assert_eq!(body.week_history, None);
    }

    #[sqlx::test]
    async fn test_ok_with_secret(pool: sqlx::PgPool) {
        setup_db(&pool).await;

        let app = app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .header(TRAIL_SECRET_HEADER, "wow")
                    .uri("/info/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response
            .into_body()
            .deserialize_json::<TTResponse<TrailInfoResponseData>>()
            .await;

        assert!(matches!(body, TTResponse::Data(_)));
        let body = body.unwrap_data();

        assert_eq!(body.trailid, "test");
        assert_eq!(body.long, "https://example.com");
        assert_eq!(body.unique_tracks, 6);
        assert_eq!(body.total_tracks, 9);

        assert_eq!(body.expiration_hours, Some(1));
        assert!(body.expires_at.is_some());
        assert!(body.created_at.is_some());

        assert_eq!(body.week_history, None);
    }

    #[sqlx::test]
    async fn test_not_found(pool: sqlx::PgPool) {
        let app = app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/info/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.into_body().to_string().await;
        assert_eq!(body, TRAIL_NOT_FOUND_OR_EXPIRED_MSG);
    }

    #[sqlx::test]
    async fn test_ok_with_week_history(pool: sqlx::PgPool) {
        setup_db(&pool).await;

        let app = app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .header(TRAIL_SECRET_HEADER, "wow")
                    .uri("/info/test?week_history=true")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response
            .into_body()
            .deserialize_json::<TTResponse<TrailInfoResponseData>>()
            .await;

        assert!(matches!(body, TTResponse::Data(_)));
        let body = body.unwrap_data();

        assert_eq!(body.trailid, "test");
        assert_eq!(body.long, "https://example.com");
        assert_eq!(body.unique_tracks, 6);
        assert_eq!(body.total_tracks, 9);

        assert_eq!(body.expiration_hours, Some(1));
        assert!(body.expires_at.is_some());
        assert!(body.created_at.is_some());

        assert_eq!(body.week_history.map(|v| v.len()), Some(6));
    }
}
