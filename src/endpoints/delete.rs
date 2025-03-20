use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};

use super::{
    common::{SameErrorResult, TRAIL_SECRET_HEADER},
    resolve::TRAIL_NOT_FOUND_OR_EXPIRED_MSG,
};

pub enum DeleteTrailResponse {
    Unauthenticated,
    Unauthorized,
    Deleted,
    NotFound,
    InternalError(String),
}

impl From<sqlx::Error> for DeleteTrailResponse {
    fn from(err: sqlx::Error) -> Self {
        Self::InternalError(err.to_string())
    }
}

impl IntoResponse for DeleteTrailResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthenticated => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body("".into())
                .unwrap(),
            Self::Unauthorized => Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body("".into())
                .unwrap(),
            Self::Deleted => Response::builder()
                .status(StatusCode::OK)
                .body("".into())
                .unwrap(),
            Self::NotFound => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(TRAIL_NOT_FOUND_OR_EXPIRED_MSG.into())
                .unwrap(),
            Self::InternalError(msg) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(msg.into())
                .unwrap(),
        }
    }
}

pub async fn delete_trail(
    State(pool): State<sqlx::PgPool>,
    Path(trailid): Path<String>,
    headers: HeaderMap,
) -> SameErrorResult<DeleteTrailResponse> {
    let trail = sqlx::query!(
        r#"
        SELECT short, secret
        FROM trails
        WHERE short = $1
        "#,
        trailid
    )
    .fetch_optional(&pool)
    .await?;

    if trail.is_none() {
        return Ok(DeleteTrailResponse::NotFound);
    }
    let trail = trail.unwrap();

    let secret = match headers
        .get(TRAIL_SECRET_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(String::from)
    {
        Some(secret) => secret,
        None => return Ok(DeleteTrailResponse::Unauthenticated),
    };

    if secret != trail.secret {
        return Ok(DeleteTrailResponse::Unauthorized);
    }

    sqlx::query!(
        r#"
        DELETE FROM trails
        WHERE short = $1
        "#,
        trailid
    )
    .execute(&pool)
    .await?;

    Ok(DeleteTrailResponse::Deleted)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{app::app, endpoints::common::TRAIL_SECRET_HEADER, utils::testing::BodyToString};

    #[sqlx::test]
    async fn test_not_authenticated(pool: sqlx::PgPool) {
        sqlx::query!(
            r#"
            INSERT INTO trails (short, long, expiration_hours, secret)
            VALUES ('test', 'https://example.com', 1, 'wow')
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let app = app(pool.clone());

        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/t/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "{}",
            response.into_body().to_string().await
        );
    }

    #[sqlx::test]
    async fn test_not_authorized(pool: sqlx::PgPool) {
        sqlx::query!(
            r#"
            INSERT INTO trails (short, long, expiration_hours, secret)
            VALUES ('test', 'https://example.com', 1, 'wow')
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let app = app(pool.clone());

        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .header(TRAIL_SECRET_HEADER, "notwow")
                    .uri("/t/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::FORBIDDEN,
            "{}",
            response.into_body().to_string().await
        );
    }

    #[sqlx::test]
    async fn test_not_found(pool: sqlx::PgPool) {
        let app = app(pool.clone());

        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri("/t/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{}",
            response.into_body().to_string().await
        );
    }

    #[sqlx::test]
    async fn test_ok(pool: sqlx::PgPool) {
        sqlx::query!(
            r#"
            INSERT INTO trails (short, long, expiration_hours, secret)
            VALUES ('test', 'https://example.com', 1, 'wow')
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let app = app(pool.clone());

        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .header(TRAIL_SECRET_HEADER, "wow")
                    .uri("/t/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "{}",
            response.into_body().to_string().await
        );

        let trail = sqlx::query!(
            r#"
            SELECT short
            FROM trails
            WHERE short = 'test'
            "#
        )
        .fetch_optional(&pool)
        .await
        .unwrap();

        assert!(trail.is_none());
    }
}
