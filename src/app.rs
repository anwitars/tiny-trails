use std::net::SocketAddr;

use crate::endpoints;
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;

// Only used for testing, but can not compile only in test mode, because of runtime checks in app
// function
pub const MOCK_IP: SocketAddr = SocketAddr::V4(std::net::SocketAddrV4::new(
    std::net::Ipv4Addr::new(127, 0, 0, 1),
    3000,
));

pub fn app(pool: SqlitePool) -> Router<()> {
    let mut router = Router::new()
        .route("/ping", get(endpoints::ping))
        .route("/shorten", post(endpoints::shorten))
        .route("/t/{trailid}", get(endpoints::resolve))
        .route("/info/{trailid}", get(endpoints::trail_info))
        .with_state(pool);

    if cfg!(test) {
        use axum::extract::connect_info::MockConnectInfo;

        router = router.layer(MockConnectInfo(MOCK_IP));
    }

    router
}
