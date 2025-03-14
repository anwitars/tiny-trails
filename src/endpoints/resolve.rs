use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
};

use super::common::SameErrorResult;

const NOT_FOUND_OR_EXPIRED_MSG: &str = "Trail ID has not been found or has expired";

pub enum ResolveResponse {
    Found(String),
    NotFound,
    Expired,
    InternalError(String),
}

impl IntoResponse for ResolveResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Found(url) => Response::builder()
                .status(StatusCode::FOUND)
                .header("Location", url)
                .body("".into())
                .unwrap(),
            Self::NotFound | Self::Expired => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOT_FOUND_OR_EXPIRED_MSG.into())
                .unwrap(),
            Self::InternalError(msg) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(msg.into())
                .unwrap(),
        }
    }
}

impl From<sqlx::Error> for ResolveResponse {
    fn from(err: sqlx::Error) -> Self {
        Self::InternalError(err.to_string())
    }
}

pub async fn resolve(
    State(pool): State<sqlx::SqlitePool>,
    Path(trailid): Path<String>,
) -> SameErrorResult<ResolveResponse> {
    log::debug!("Resolving trail ID: {}", trailid);

    if trailid.is_empty() {
        return Ok(ResolveResponse::NotFound);
    }

    let long_url = sqlx::query!(
        r#"
        SELECT long, created_at, expiration_hours
        FROM trails
        WHERE short = ?
        "#,
        trailid
    )
    .fetch_optional(&pool)
    .await?;

    log::debug!("Resolved trail ID: {:?}", long_url);

    match long_url {
        Some(record) => {
            let now = chrono::Utc::now().naive_utc();
            let expires_at = record.created_at + chrono::Duration::hours(record.expiration_hours);

            if expires_at < now {
                let expired_ago = chrono_humanize::HumanTime::from(expires_at - now);
                log::debug!("Trail ID '{}' expired {}", trailid, expired_ago);
                return Ok(ResolveResponse::Expired);
            }

            Ok(ResolveResponse::Found(record.long))
        }
        None => {
            log::debug!("Trail ID not found: {}", trailid);
            Ok(ResolveResponse::NotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{
        app,
        utils::testing::{get_test_pool, init_logging},
    };

    #[tokio::test]
    async fn test_resolve() {
        init_logging();
        let pool = get_test_pool().await;

        sqlx::query!(
            r#"
            INSERT INTO trails (short, long, expiration_hours)
            VALUES ('test', 'https://example.com', 1)
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let app = app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/t/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FOUND);

        let location = response.headers().get("Location").unwrap();
        assert_eq!(location, "https://example.com");
    }

    #[tokio::test]
    async fn test_not_found() {
        init_logging();
        let pool = get_test_pool().await;
        let app = app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/t/notexists")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_expired() {
        init_logging();
        let pool = get_test_pool().await;

        let created_at = chrono::Utc::now().naive_utc() - chrono::Duration::hours(2);
        sqlx::query!(
            r#"
            INSERT INTO trails (short, long, expiration_hours, created_at)
            VALUES ('expired', 'https://example.com', 1, ?)
            "#,
            created_at
        )
        .execute(&pool)
        .await
        .unwrap();

        let app = app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/t/expired")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
