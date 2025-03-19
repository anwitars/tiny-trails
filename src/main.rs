use std::net::SocketAddr;

use clap::Parser;
use constcat::concat;
use tiny_trails::{app, utils::env::TT_ENV_PREFIX};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, clap::Parser)]
struct AppArgs {
    #[clap(long, default_value = "0.0.0.0", env = concat!(TT_ENV_PREFIX, "HOST"))]
    pub host: String,

    #[clap(short, long, default_value = "3000", env = concat!(TT_ENV_PREFIX, "PORT"))]
    pub port: u16,

    #[clap(short, long, env = concat!(TT_ENV_PREFIX, "DATABASE"))]
    pub database: String,
}

impl AppArgs {
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let app_args = AppArgs::parse();
    log::debug!("App args: {:?}", app_args);

    let pool = sqlx::PgPool::connect(&app_args.database).await.unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let listen_address = app_args.listen_address();
    log::info!("Listening on {}", listen_address);

    let listener = tokio::net::TcpListener::bind(listen_address).await.unwrap();

    axum::serve(
        listener,
        app(pool).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
