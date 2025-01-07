use anyhow::Result;
use axum::{http::Method, routing::post, Router};
use fuscion::{signin_handler, signup_handler, AppConfig, AppState};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{
    fmt::Layer, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer as _,
};

const ADDR: &str = "127.0.0.1:";

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::new()?;
    let addr = format!("{}{}", ADDR, config.server.port);

    let state = AppState::new(config).await?;

    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);
    registry().with(layer).init();

    info!("Server listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
        ])
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);

    let app = Router::new()
        .route("/register", post(signup_handler))
        .route("/login", post(signin_handler))
        .with_state(state)
        .layer(cors);

    axum::serve(listener, app).await?;

    Ok(())
}
