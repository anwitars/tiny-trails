use axum::response::IntoResponse;

use crate::validation;

/// Unified response type for the application.
///
/// Example for a successful response:
/// ```json
/// {
///    "data": {
///        "id": 1,
///        "name": "John Doe"
///    }
/// }
/// ```
///
/// Example for an error response:
/// ```json
/// {
///   "errors": [
///       {
///           "message": "Required field",
///           "location": ["name"]
///       }
///   ]
/// }
/// ```
#[derive(Debug)]
pub enum TTResponse<T>
where
    T: serde::Serialize,
{
    Data(T),
    Error(validation::Error),
    Errors(Vec<validation::Error>),
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

impl<'a, T> serde::Deserialize<'a> for TTResponse<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn deserialize<D>(deserializer: D) -> Result<TTResponse<T>, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;

        if value.get("data").is_some() {
            let data =
                serde_json::from_value(value["data"].clone()).map_err(serde::de::Error::custom)?;
            Ok(TTResponse::Data(data))
        } else if value.get("errors").is_some() {
            let errors = serde_json::from_value(value["errors"].clone())
                .map_err(serde::de::Error::custom)?;
            Ok(TTResponse::Errors(errors))
        } else {
            Err(serde::de::Error::custom("Invalid response"))
        }
    }
}

impl<T> From<Vec<validation::Error>> for TTResponse<T>
where
    T: serde::Serialize,
{
    fn from(errors: Vec<validation::Error>) -> Self {
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

impl<T> TTResponse<T>
where
    T: serde::Serialize,
{
    /// Extracts the data from the response if the response is [TTResponse::Data].
    pub fn unwrap_data(self) -> T {
        match self {
            Self::Data(data) => data,
            _ => panic!("Expected TTResponse::Data"),
        }
    }

    /// Extracts the errors from the response if the response is [TTResponse::Error].
    pub fn unwrap_errors(self) -> Vec<validation::Error> {
        match self {
            Self::Errors(errors) => errors,
            Self::Error(error) => vec![error],
            Self::Data(_) => panic!("Expected TTResponse::Errors or TTResponse::Error"),
        }
    }
}
