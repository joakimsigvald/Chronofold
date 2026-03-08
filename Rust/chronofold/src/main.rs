use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{Router, response::Html, routing::get};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/",
            get(|| async { Html(include_str!("../templates/index.html")) }),
        )
        .route(
            "/simulation",
            get(|| async { Html(include_str!("../templates/simulation.html")) }),
        )
        .nest_service("/assets", ServeDir::new("assets")) // Generic handler
        .route("/ws", get(ws_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("--- Chronofold Observer: http://127.0.0.1:3000 ---");

    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_handshake)
}

async fn handle_handshake(mut socket: WebSocket) {
    let mut tick_counter = 0;

    // Send the first number to start the chain
    if socket
        .send(Message::Text(tick_counter.to_string()))
        .await
        .is_err()
    {
        return;
    }

    loop {
        // Wait indefinitely for the frontend ACK
        if let Some(Ok(Message::Text(text))) = socket.recv().await {
            if text == "ACK" {
                tick_counter += 1;

                // Add a small delay so it doesn't instantly count to a million
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                if socket
                    .send(Message::Text(tick_counter.to_string()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        } else {
            break; // Connection lost
        }
    }
}
