use super::common::SameErrorResult;
use crate::encoding::hash_with_env_salt;
use axum::{
    extract::{ConnectInfo, Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
};
use std::net::SocketAddr;

pub const TRAIL_NOT_FOUND_OR_EXPIRED_MSG: &str = "Trail ID has not been found or has expired";

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
                .body(TRAIL_NOT_FOUND_OR_EXPIRED_MSG.into())
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
    State(pool): State<sqlx::PgPool>,
    Path(trailid): Path<String>,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
) -> SameErrorResult<ResolveResponse> {
    log::debug!("Resolving trail ID: {}", trailid);

    if trailid.is_empty() {
        return Ok(ResolveResponse::NotFound);
    }

    let long_url = sqlx::query!(
        r#"
        SELECT id, long, created_at, expiration_hours
        FROM trails
        WHERE short = $1
        "#,
        trailid
    )
    .fetch_optional(&pool)
    .await?;

    match long_url {
        Some(record) => {
            let now = chrono::Utc::now().naive_utc();
            let expires_at =
                record.created_at + chrono::Duration::hours(record.expiration_hours as i64);

            if expires_at < now {
                let expired_ago = chrono_humanize::HumanTime::from(expires_at - now);
                log::debug!("Trail ID '{}' expired {}", trailid, expired_ago);
                return Ok(ResolveResponse::Expired);
            }

            let remote_addr = hash_with_env_salt(&address.to_string());
            log::debug!(
                "Resolved trail ID '{}' to '{}' for {}",
                trailid,
                record.long,
                remote_addr
            );

            sqlx::query!(
                r#"
                INSERT INTO tracks (trail_id, hashed_ip, created_at) VALUES ($1, $2, $3)
                "#,
                record.id,
                remote_addr,
                now
            )
            .execute(&pool)
            .await?;

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
        app::{MOCK_IP, app},
        encoding::hash_with_env_salt,
        utils::testing::BodyToString,
    };

    #[sqlx::test]
    async fn test_resolve(pool: sqlx::PgPool) {
        sqlx::query!(
            r#"
            INSERT INTO trails (short, long, expiration_hours, secret)
            VALUES ('test', 'https://example.com', 1, '')
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        let app = app(pool.clone());

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/t/test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(
            response.status(),
            StatusCode::FOUND,
            "{}",
            response.into_body().to_string().await
        );

        let location = response.headers().get("Location").unwrap();
        assert_eq!(location, "https://example.com");

        let tracks = sqlx::query!(
            r#"
            SELECT trail_id, hashed_ip
            FROM tracks
            WHERE trail_id = 1
            "#
        )
        .fetch_all(&pool)
        .await
        .unwrap();

        assert_eq!(tracks.len(), 1);
        let track = &tracks[0];

        assert_eq!(track.trail_id, 1);
        assert_eq!(
            track.hashed_ip,
            Some(hash_with_env_salt(&MOCK_IP.to_string()))
        );
    }

    #[sqlx::test]
    async fn test_not_found(pool: sqlx::PgPool) {
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

    #[sqlx::test]
    async fn test_expired(pool: sqlx::PgPool) {
        let created_at = chrono::Utc::now().naive_utc() - chrono::Duration::hours(2);
        sqlx::query!(
            r#"
            INSERT INTO trails (short, long, expiration_hours, created_at, secret)
            VALUES ('expired', 'https://example.com', 1, $1, '')
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

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{}",
            response.into_body().to_string().await
        );
    }
}
