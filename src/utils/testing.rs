use async_trait::async_trait;
use http_body_util::BodyExt;

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
