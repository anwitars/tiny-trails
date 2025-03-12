use axum::{
    extract::{Path, State},
    http::{Response, StatusCode},
    response::IntoResponse,
};

use super::common::SameErrorResult;

const NOT_FOUND_MSG: &str = "Trail ID has not been found";

pub enum ResolveResponse {
    Found(String),
    NotFound,
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
            Self::NotFound => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOT_FOUND_MSG.into())
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
        SELECT long, expiration_hours
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
            let now = chrono::Utc::now();
            let expires_at = now + chrono::Duration::hours(record.expiration_hours);

            if expires_at < now {
                log::debug!("Trail ID has expired: {}", trailid);
                return Ok(ResolveResponse::NotFound);
            }

            Ok(ResolveResponse::Found(record.long))
        }
        None => {
            log::debug!("Trail ID not found: {}", trailid);
            Ok(ResolveResponse::NotFound)
        }
    }
}
