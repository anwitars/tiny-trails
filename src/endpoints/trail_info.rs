use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};

use crate::response::TTResponse;

use super::{common::SameErrorResult, resolve::TRAIL_NOT_FOUND_OR_EXPIRED_MSG};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TrailInfoResponseData {
    pub trailid: String,
    pub long: String,
    pub created_at: String,
    pub expiration_hours: i32,
    pub expires_at: String,
    pub unique_tracks: i64,
    pub total_tracks: i64,
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
) -> SameErrorResult<TrailInfoResponse> {
    let trail = sqlx::query!(
        r#"
        SELECT id, long, created_at, expiration_hours
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

    Ok(TrailInfoResponse::TTResponse(TTResponse::Data(
        TrailInfoResponseData {
            trailid,
            long: trail.long,
            created_at: trail.created_at.to_string(),
            expiration_hours: trail.expiration_hours,
            expires_at: (trail.created_at + chrono::Duration::hours(trail.expiration_hours as i64))
                .to_string(),
            unique_tracks: track_info.unique_tracks.unwrap_or(0),
            total_tracks: track_info.total_tracks.unwrap_or(0),
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

    #[sqlx::test]
    async fn test_ok(pool: sqlx::PgPool) {
        let trail_db_id = sqlx::query!(
            r#"
            INSERT INTO trails (short, long, expiration_hours)
            VALUES ('test', 'https://example.com', 1)
            RETURNING id
            "#
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .id;

        async fn insert_track(pool: &sqlx::PgPool, trail_db_id: i32, hashed_ip: Option<&str>) {
            let hashed_ip = hashed_ip.map(String::from);

            sqlx::query!(
                r#"
                INSERT INTO tracks (trail_id, hashed_ip)
                VALUES ($1, $2)
                "#,
                trail_db_id,
                hashed_ip
            )
            .execute(pool)
            .await
            .unwrap();
        }

        insert_track(&pool, trail_db_id, Some("one")).await;
        insert_track(&pool, trail_db_id, Some("two")).await;
        insert_track(&pool, trail_db_id, Some("two")).await;
        insert_track(&pool, trail_db_id, Some("three")).await;
        insert_track(&pool, trail_db_id, Some("three")).await;
        insert_track(&pool, trail_db_id, Some("three")).await;

        insert_track(&pool, trail_db_id, None).await;
        insert_track(&pool, trail_db_id, None).await;
        insert_track(&pool, trail_db_id, None).await;

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
        assert_eq!(body.expiration_hours, 1);
        assert_eq!(body.unique_tracks, 6);
        assert_eq!(body.total_tracks, 9);
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
}
