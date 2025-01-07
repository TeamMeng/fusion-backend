use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("argon2 password hash error: {0}")]
    Argon2Error(#[from] argon2::password_hash::Error),

    #[error("io error: {0}")]
    IoError(#[from] io::Error),

    #[error("serde yaml error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("{0}")]
    ServerError(String),
}

#[derive(Debug, Serialize)]
struct OutputError {
    pub error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::Argon2Error(_) => StatusCode::BAD_REQUEST,
            Self::IoError(_) | Self::SerdeYamlError(_) | Self::SqlxError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::ServerError(_) => StatusCode::BAD_REQUEST,
        };

        (status, Json(OutputError::new(self.to_string()))).into_response()
    }
}

impl OutputError {
    fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}
