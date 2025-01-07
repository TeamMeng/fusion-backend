mod config;
mod error;
mod model;

use sqlx::PgPool;
use sqlx_db_tester::TestPg;
use std::{ops::Deref, path::Path, sync::Arc};

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

    pub async fn new_for_test() -> Result<(TestPg, Self), AppError> {
        let config = AppConfig::new()?;
        let ret = config.server.db_url.rfind('/');

        let url = match ret {
            Some(post) => &config.server.db_url[..post],
            None => "postgres://postgres:postgres@localhost:5432",
        };

        let tdb = TestPg::new(url.to_string(), Path::new("./migrations"));
        let pool = tdb.get_pool().await;

        Ok((
            tdb,
            Self {
                inner: Arc::new(AppStateInner::new(pool, config)),
            },
        ))
    }
}

impl AppStateInner {
    fn new(pool: PgPool, config: AppConfig) -> Self {
        Self { pool, config }
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
