
use tokio::net::TcpListener;
use axum::{Router, routing::{get, post}, Json};
use shared::{LoginRequest, AuthResponse};



#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(health))
    .route("/login", post(login));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "server alive"
}


async fn login(Json(req): Json<LoginRequest>) -> Json<AuthResponse> {
    // FAKE check for now — just to prove the round-trip.
    // Step 3 replaces this with a real DB lookup + password verify.
    if req.username == "admin" && req.password == "1234" {
        Json(AuthResponse::Success)
    } else {
        Json(AuthResponse::InvalidCredentials)
    }
}
