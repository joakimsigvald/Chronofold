use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::{Html, IntoResponse};

use crate::engine::ChronofoldEngine;
use crate::models::VacuumState;

pub async fn index_html() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

pub async fn simulation_html() -> Html<&'static str> {
    Html(include_str!("../templates/simulation.html"))
}

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(run_simulation)
}

async fn send_state(state: &VacuumState, socket: &mut WebSocket) -> Result<(), ()> {
    let json = serde_json::to_string(state).unwrap();
    socket.send(Message::Text(json)).await.map_err(|_| ())
}

async fn run_simulation(mut socket: WebSocket) {
    // Initialize the Big Bang
    let mut engine = ChronofoldEngine::ignite();

    // Send initial state
    if send_state(engine.state(), &mut socket).await.is_err() {
        return;
    }

    // The Observer Loop
    loop {
        let text = socket
            .recv()
            .await
            .and_then(Result::ok)
            .and_then(|msg| match msg {
                Message::Text(t) => Some(t),
                _ => None,
            });

        match text.as_deref() {
            Some("ACK") => {
                engine.advance();
                if send_state(engine.state(), &mut socket).await.is_err() {
                    break;
                }
            }
            Some(unrecognized) => panic!(
                "Critical Causal Error: Unrecognized signal from Observer: '{}'",
                unrecognized
            ),
            None => break,
        }
    }
}
