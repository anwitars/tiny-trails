use std::net::SocketAddr;

use clap::Parser;
use tiny_trails::{app, utils::env::TT_ENV_PREFIX, value_from_env};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, clap::Parser)]
struct Cli {
    #[clap(long)]
    pub host: Option<String>,

    #[clap(short, long)]
    pub port: Option<u16>,

    #[clap(short, long)]
    pub database: Option<String>,
}

#[derive(Debug, Clone)]
struct AppArgs {
    pub host: String,
    pub port: u16,
    pub database: String,
}

impl AppArgs {
    pub fn from_env_and_cli(cli: &Cli) -> Self {
        let env_host = value_from_env!("HOST");
        let env_port = value_from_env!("PORT", u16);
        let env_database = value_from_env!("DATABASE");

        let host = cli
            .host
            .clone()
            .unwrap_or(env_host.unwrap_or("0.0.0.0".to_string()));
        let port = cli.port.unwrap_or(env_port.unwrap_or(3000));
        let database = cli.database.clone().unwrap_or_else(|| {
            env_database.unwrap_or_else(|| panic!("Database must be set via environment or CLI"))
        });

        Self {
            host,
            port,
            database,
        }
    }

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

    let cli = Cli::parse();
    let app_args = AppArgs::from_env_and_cli(&cli);
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
