use async_trait::async_trait;
use http_body_util::BodyExt;

pub async fn get_test_pool() -> sqlx::SqlitePool {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

pub fn init_logging() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    });
}

#[async_trait]
pub trait BodyDeserializeJson {
    async fn deserialize_json<T: serde::de::DeserializeOwned>(self) -> T;
}

#[async_trait]
impl BodyDeserializeJson for axum::body::Body {
    async fn deserialize_json<T: serde::de::DeserializeOwned>(self) -> T {
        let body = self.collect().await.unwrap().to_bytes();
        let body = String::from_utf8(body.to_vec()).unwrap();

        match serde_json::from_str(&body) {
            Ok(data) => data,
            Err(e) => panic!("Failed to deserialize body: {} | Body: {}", e, body),
        }
    }
}
