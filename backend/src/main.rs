use hyper::Server;
use axum::{
    routing::post,
    Router,
    Json,
    extract::State,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct GuessRequest {
    guess: u32,
}

#[derive(Debug, Serialize)]
struct GuessResponse {
    message: String,
    status: String,
}

#[derive(Clone)]
struct AppState {
    secret_number: Mutex<u32>,
}

#[tokio::main]
async fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {}", secret_number);

    let app_state = AppState {
        secret_number: Mutex::new(secret_number),
    };

    let app = Router::new()
        .route("/guess", post(guess_handler))
        .with_state(app_state);

    println!("Server running on http://localhost:3000");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn guess_handler(
    State(state): State<AppState>,
    Json(payload): Json<GuessRequest>,
) -> Json<GuessResponse> {
    let secret_number = *state.secret_number.lock().unwrap();
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
