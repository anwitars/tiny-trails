use axum::response::IntoResponse;

use crate::response::TTResponse;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Error {
    pub message: String,
    pub location: Vec<String>,
}

impl Error {
    pub fn new(message: String, location: Vec<String>) -> Self {
        Self { message, location }
    }

    pub fn extra_not_allowed(location: Vec<String>) -> Self {
        Self {
            message: "Extra field not allowed".to_string(),
            location,
        }
    }

    pub fn required_field(location: Vec<String>) -> Self {
        Self {
            message: "Required field".to_string(),
            location,
        }
    }

    pub fn type_mismatch(expected: &str, location: Vec<String>) -> Self {
        Self {
            message: format!("Field must be type {}", expected),
            location,
        }
    }

    pub fn internal_error(msg: impl ToString) -> Self {
        Self {
            message: msg.to_string(),
            location: vec![":internal:".to_string()],
        }
    }

    pub fn invalid_url(location: Vec<String>) -> Self {
        Self {
            message: "Invalid URL".to_string(),
            location,
        }
    }

    pub fn max_exceeded(max: i64, location: Vec<String>) -> Self {
        Self {
            message: format!("Value must be at most: {}", max),
            location,
        }
    }

    pub fn min_exceeded(min: i64, location: Vec<String>) -> Self {
        Self {
            message: format!("Value must be at least: {}", min),
            location,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap();
        axum::http::Response::builder()
            .status(axum::http::StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(body.into())
            .unwrap()
    }
}

impl<T> Into<TTResponse<T>> for Error
where
    T: serde::Serialize,
{
    fn into(self) -> TTResponse<T> {
        TTResponse::Error(self)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::internal_error(err)
    }
}
