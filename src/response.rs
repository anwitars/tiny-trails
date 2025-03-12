use axum::response::IntoResponse;

#[derive(Debug)]
pub enum TTResponse<T>
where
    T: serde::Serialize,
{
    Data(T),
    Error(crate::validation::Error),
    Errors(Vec<crate::validation::Error>),
}

impl<T> IntoResponse for TTResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Data(data) => {
                let body = serde_json::json!({ "data": data });
                axum::http::Response::builder()
                    .status(axum::http::StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(body.to_string().into())
                    .unwrap()
            }
            Self::Error(error) => {
                let is_internal = error
                    .location
                    .first()
                    .map_or(false, |loc| loc == ":internal:");
                let status = if is_internal {
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                } else {
                    axum::http::StatusCode::BAD_REQUEST
                };

                let body = serde_json::json!({ "errors": [error] });

                axum::http::Response::builder()
                    .status(status)
                    .header("Content-Type", "application/json")
                    .body(body.to_string().into())
                    .unwrap()
            }
            Self::Errors(errors) => {
                let internal_error = errors.iter().find(|error| {
                    error
                        .location
                        .first()
                        .map_or(false, |loc| loc == ":internal:")
                });

                let status = if internal_error.is_some() {
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                } else {
                    axum::http::StatusCode::BAD_REQUEST
                };

                let errors = if let Some(internal_error) = internal_error {
                    log::error!("Internal error: {:?}", internal_error);
                    vec![internal_error.clone()]
                } else {
                    errors
                };

                let body = serde_json::json!({ "errors": errors });

                axum::http::Response::builder()
                    .status(status)
                    .header("Content-Type", "application/json")
                    .body(body.to_string().into())
                    .unwrap()
            }
        }
    }
}

impl<T> From<Vec<crate::validation::Error>> for TTResponse<T>
where
    T: serde::Serialize,
{
    fn from(errors: Vec<crate::validation::Error>) -> Self {
        TTResponse::Errors(errors)
    }
}

impl<T> From<sqlx::Error> for TTResponse<T>
where
    T: serde::Serialize,
{
    fn from(value: sqlx::Error) -> Self {
        TTResponse::Error(value.into())
    }
}
