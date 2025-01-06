use anyhow::Result;
use axum::{http::Method, response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:3000").await?;
    println!("Server listening on localhost:3000");

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
        .route("/register", post(index_handler))
        .layer(cors);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn index_handler(Json(input): Json<User>) -> impl IntoResponse {
    println!("{:?}", input);
    Json(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn func() {
        assert_eq!(1, 1)
    }
}
