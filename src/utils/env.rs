/// The prefix all Tiny Trails related environment variables should have.
pub const TT_ENV_PREFIX: &str = "TT_";

/// Concatenates the environment variable prefix with the given key.
#[macro_export]
macro_rules! prefixed_env {
    ($key:expr) => {{
        constcat::concat!($crate::utils::env::TT_ENV_PREFIX, $key)
    }};
}
