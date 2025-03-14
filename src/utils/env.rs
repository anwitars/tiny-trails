pub const TT_ENV_PREFIX: &str = "TT_";

#[macro_export]
macro_rules! value_from_env {
    ($key:expr) => {{
        use constcat::concat;

        let value = std::env::var(concat!(TT_ENV_PREFIX, $key)).unwrap_or_default();
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    }};

    ($key:expr, $type:ty) => {{
        use constcat::concat;

        let value = std::env::var(concat!(TT_ENV_PREFIX, $key)).unwrap_or_default();
        if value.is_empty() {
            None
        } else {
            Some(value.parse::<$type>().unwrap())
        }
    }};
}

#[macro_export]
macro_rules! env_with_prefix {
    ($key:expr) => {{
        use constcat::concat;

        concat!(TT_ENV_PREFIX, $key)
    }};
}
