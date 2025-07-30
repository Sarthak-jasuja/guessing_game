use axum::{
    routing::post,
    Router,
    Json,
    http::{Method},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use rand::Rng;
use once_cell::sync::Lazy;  // ✅ For global state
use std::sync::Mutex;

// ✅ Global random number stored in Mutex
static SECRET_NUMBER: Lazy<Mutex<u32>> = Lazy::new(|| {
    let num = rand::thread_rng().gen_range(1..=100);
    println!("Generated secret number: {}", num);
    Mutex::new(num)
});

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
    let mut number = SECRET_NUMBER.lock().unwrap();  // 🔐 Access shared number
    let response = if payload.guess < *number {
        GuessResponse {
            message: "Too small!".to_string(),
            status: "continue".to_string(),
        }
    } else if payload.guess > *number {
        GuessResponse {
            message: "Too big!".to_string(),
            status: "continue".to_string(),
        }
    } else {
        let new_number = rand::thread_rng().gen_range(1..=100);
        println!("New secret number: {}", new_number);
        *number = new_number;  // 🎯 Reset after win

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
