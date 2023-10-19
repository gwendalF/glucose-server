use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database,
    #[error("Network issue")]
    Network,
    #[error("Parsing error, check the data: {0}")]
    Parsing(String),
    #[error("Access denied")]
    AccessDenied,
}

impl From<sqlx::Error> for AppError {
    fn from(_: sqlx::Error) -> Self {
        Self::Database
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::Database | AppError::Network => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppError::AccessDenied => (StatusCode::FORBIDDEN, self.to_string()),
            AppError::Parsing(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };
        let body = Json(json!({
            "error": message
        }));
        (status, body).into_response()
    }
}
