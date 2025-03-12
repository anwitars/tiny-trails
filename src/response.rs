use axum::response::IntoResponse;

// TODO: somehow support ? operator
#[macro_export]
macro_rules! return_if_errors {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(errors) => return TTResponse::errors(errors),
        }
    };
}

#[macro_export]
macro_rules! return_if_error {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(error) => return TTResponse::error(error.into()),
        }
    };
}

#[derive(Debug, serde::Serialize)]
pub struct TTResponse<T>
where
    T: serde::Serialize,
{
    pub data: Option<T>,
    pub errors: Option<Vec<crate::validation::Error>>,
}

impl<T> TTResponse<T>
where
    T: serde::Serialize,
{
    pub fn data(data: T) -> Self {
        Self {
            data: Some(data),
            errors: None,
        }
    }

    pub fn error(error: crate::validation::Error) -> Self {
        Self {
            data: None,
            errors: Some(vec![error]),
        }
    }

    pub fn errors(errors: Vec<crate::validation::Error>) -> Self {
        Self {
            data: None,
            errors: Some(errors),
        }
    }
}

impl<T> IntoResponse for TTResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match (self.data, self.errors) {
            (Some(data), None) => {
                let body = serde_json::json!({ "data": data });
                axum::http::Response::builder()
                    .status(axum::http::StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(body.to_string().into())
                    .unwrap()
            }
            (None, Some(errors)) => {
                let internal_error = errors
                    .iter()
                    .find(|e| e.location.contains(&":internal:".to_string()));
                let status = if internal_error.is_some() {
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR
                } else {
                    axum::http::StatusCode::BAD_REQUEST
                };

                let errors = match internal_error {
                    Some(e) => vec![e.clone()],
                    None => errors,
                };

                let body = serde_json::json!({ "errors": errors });
                axum::http::Response::builder()
                    .status(status)
                    .header("Content-Type", "application/json")
                    .body(body.to_string().into())
                    .unwrap()
            }
            _ => {
                // NOTE: This should never happen
                let error = crate::validation::Error::internal_error("Invalid response");
                let body = serde_json::json!({ "errors": [error] });
                axum::http::Response::builder()
                    .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "application/json")
                    .body(body.to_string().into())
                    .unwrap()
            }
        }
    }
}
