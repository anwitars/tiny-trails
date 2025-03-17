pub async fn ping() -> &'static str {
    "pong"
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{app, utils::testing::BodyToString};

    #[sqlx::test]
    async fn test_ping(pool: sqlx::PgPool) {
        let app = app(pool);

        let response = app
            .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().to_string().await;

        assert_eq!(body, "pong");
    }
}
