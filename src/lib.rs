mod config;
mod error;

use sqlx::PgPool;
use std::sync::Arc;

pub use config::{AppConfig, AuthConfig, ServerConfig};
pub use error::AppError;

#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub struct AppStateInner {
    pub pool: PgPool,
    pub config: AppConfig,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        let pool = PgPool::connect(&config.server.db_url).await?;
        Ok(Self {
            inner: Arc::new(AppStateInner::new(pool, config)),
        })
    }
}

impl AppStateInner {
    fn new(pool: PgPool, config: AppConfig) -> Self {
        Self { pool, config }
    }
}
