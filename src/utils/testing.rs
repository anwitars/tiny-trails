use crate::utils::env::TT_ENV_PREFIX;
use async_trait::async_trait;
use http_body_util::BodyExt;

use crate::env_with_prefix;

pub async fn get_test_pool() -> sqlx::PgPool {
    let pool = sqlx::PgPool::connect(env_with_prefix!("TEST_DATABASE_URL"))
        .await
        .unwrap();
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
pub trait BodyToString {
    async fn to_string(self) -> String;
}

#[async_trait]
impl BodyDeserializeJson for axum::body::Body {
    async fn deserialize_json<T: serde::de::DeserializeOwned>(self) -> T {
        let body = self.to_string().await;

        match serde_json::from_str(&body) {
            Ok(data) => data,
            Err(e) => panic!("Failed to deserialize body: {} | Body: {}", e, body),
        }
    }
}

#[async_trait]
impl BodyToString for axum::body::Body {
    async fn to_string(self) -> String {
        let body = self.collect().await.unwrap().to_bytes();
        String::from_utf8(body.to_vec()).unwrap()
    }
}
