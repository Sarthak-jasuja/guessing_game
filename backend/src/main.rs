use axum::{
    routing::post,
    Router,
    Json,
    http::{Method, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use rand::Rng;  // ✅ Add this for random number

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
    let secret_number = rand::thread_rng().gen_range(1..=100);  // ✅ Random number 1 to 100
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
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/guess", post(guess_handler))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
