use crate::prefixed_env;

/// The command line arguments of the application.
/// Using the `clap` crate, we can define the command line arguments and parse them with ease.
#[derive(Debug, clap::Parser)]
pub struct AppArgs {
    /// The host to listen on.
    #[clap(long, default_value = "0.0.0.0", env = prefixed_env!("HOST"))]
    pub host: String,

    /// The port to listen on.
    #[clap(short, long, default_value = "3000", env = prefixed_env!("PORT"))]
    pub port: u16,

    /// Full database URI.
    #[clap(short, long, env = prefixed_env!("DATABASE"))]
    pub database: String,
}

impl AppArgs {
    /// Gets the listen address in the format `host:port`.
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
