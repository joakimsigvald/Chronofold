mod models;
mod engine;
mod handlers;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::index_html))
        .route("/simulation", get(handlers::simulation_html))
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/ws", get(handlers::ws_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("--- Chronofold Observer: http://127.0.0.1:3000 ---");

    axum::serve(listener, app).await.unwrap();
}