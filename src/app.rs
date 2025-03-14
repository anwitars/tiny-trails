use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;

use crate::endpoints;

pub fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/ping", get(endpoints::ping))
        .route("/shorten", post(endpoints::shorten))
        .route("/t/{trailid}", get(endpoints::resolve))
        .with_state(pool)
}
