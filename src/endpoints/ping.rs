pub async fn ping() -> &'static str {
    "pong"
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::app;

    #[tokio::test]
    async fn test_ping() {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();

        let app = app(pool);

        let response = app
            .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body, "pong");
    }
}
