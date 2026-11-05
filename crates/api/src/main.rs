use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind API listener");

    println!("API listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("API server failed");
}

async fn health() -> &'static str {
    "ok"
}
