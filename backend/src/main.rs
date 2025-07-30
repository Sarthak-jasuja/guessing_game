use std::env;
use axum::{
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[derive(Debug, Deserialize)]
struct GuessRequest {
    guess: u32,
}

#[derive(Debug, Serialize)]
struct GuessResponse {
    message: String,
    status: String,
}

async fn guess_handler(Json(payload): Json<GuessRequest>) -> Json<GuessResponse> {
    let secret_number = 42;
    let response = if payload.guess < secret_number {
        GuessResponse {
            message: "Too small!".to_string(),
            status: "continue".to_string(),
        }
    } else if payload.guess > secret_number {
        GuessResponse {
            message: "Too big!".to_string(),
            status: "continue".to_string(),
        }
    } else {
        GuessResponse {
            message: "You win!".to_string(),
            status: "win".to_string(),
        }
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    // ✅ Get port from environment variable, fallback to 3000 for local
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

    println!("Listening on http://{}", addr);

    let app = Router::new().route("/api/guess", post(guess_handler));

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
