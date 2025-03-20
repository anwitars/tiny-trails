pub const TT_ENV_PREFIX: &str = "TT_";

#[macro_export]
macro_rules! prefixed_env {
    ($key:expr) => {{ constcat::concat!($crate::utils::env::TT_ENV_PREFIX, $key) }};
}
