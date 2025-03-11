#[macro_export]
macro_rules! value_from_env {
    ($key:expr) => {{
        let value = std::env::var(concat!("TT_", $key)).unwrap_or_default();
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    }};

    ($key:expr, $type:ty) => {{
        let value = std::env::var(concat!("TT_", $key)).unwrap_or_default();
        if value.is_empty() {
            None
        } else {
            Some(value.parse::<$type>().unwrap())
        }
    }};
}
