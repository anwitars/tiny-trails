use std::net::SocketAddr;

use axum::http::{header, Method};
use clap::Parser;
use tiny_trails::{
    app, app_args::AppArgs, endpoints::TRAIL_SECRET_HEADER, utils::start_rate_limiter,
};
use tower_http::cors;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // initialize the logger with INFO level by default
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    // parse the command line arguments (with environment variables support)
    let app_args = AppArgs::parse();
    log::debug!("Application run with args: {:#?}", app_args);

    // create a connection pool to the database and run the pending migrations
    let pool = sqlx::PgPool::connect(&app_args.database).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // create a TCP listener and start the server
    let listen_address = app_args.listen_address();
    let listener = tokio::net::TcpListener::bind(&listen_address)
        .await
        .unwrap();

    // start the rate limiter thread and get the tower layer as well
    let rate_limiter = start_rate_limiter();

    log::info!("Listening on {}", listen_address);

    // serve the application
    axum::serve(
        listener,
        app(pool)
            .layer(rate_limiter)
            .layer(get_cors_layer())
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

fn get_cors_layer() -> cors::CorsLayer {
    cors::CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers([
            header::CONTENT_TYPE,
            TRAIL_SECRET_HEADER.parse::<header::HeaderName>().unwrap(),
        ])
        .max_age(std::time::Duration::from_secs(86400))
}
