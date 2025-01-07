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

    #[error("server error: {0}")]
    ServerError(String),
}
