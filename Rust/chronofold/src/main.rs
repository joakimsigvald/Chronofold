use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{Router, response::Html, routing::get};
use serde::Serialize;
use tower_http::services::ServeDir;

#[derive(Serialize, Clone)]
struct Monad {
    id: u32,
    horizon: Vec<u32>, // The Active Horizon (Relational Distance)
}

#[derive(Serialize, Clone)]
struct VacuumState {
    tick: u64,
    monads: Vec<Monad>,
    links: Vec<(u32, u32)>, // Entanglements / Triad Closures
}

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

async fn send(state: &VacuumState, socket: &mut WebSocket) -> Result<(), ()> {
    let json = serde_json::to_string(state).unwrap();
    socket.send(Message::Text(json)).await.map_err(|_| ())
}

async fn advance_tick(state: &mut VacuumState, socket: &mut WebSocket) -> Result<(), ()> {
    *state = VacuumState {
        tick: state.tick + 1,
        ..state.clone()
    };

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    send(state, socket).await
}

async fn handle_handshake(mut socket: WebSocket) {
    let mut current_state = VacuumState {
        tick: 0,
        monads: vec![
            Monad {
                id: 0,
                horizon: vec![1],
            },
            Monad {
                id: 1,
                horizon: vec![0],
            },
        ],
        links: vec![(0, 1)],
    };

    if send(&current_state, &mut socket).await.is_err() {
        return;
    }

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
                if advance_tick(&mut current_state, &mut socket).await.is_err() {
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
