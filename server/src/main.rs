use std::net::SocketAddr;
use tokio::fs;
use std::path::PathBuf;
use std::sync::Arc;
use axum::{response::Html, routing::get, Json, Router};
use axum::extract::{State, WebSocketUpgrade};
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::response::IntoResponse;
use axum::routing::post;
use tokio::sync::{broadcast, Mutex};
use serde_json::Value;

#[tokio::main]
async fn main() {
    let messages = Arc::new(Mutex::new(Vec::new()));
    let (tx, _rx) = broadcast::channel(100);

    let state = AppState {
        messages,
        broadcaster: tx,
    };

    let app = Router::new()
        .route("/", get(render_index))
        .route("/send", post(add_to_queue))
        .route("/queue", get(list_queue))
        .route("/ws", get(ws_handler))
        .with_state(Arc::new(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server listening at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    messages: Arc<Mutex<Vec<String>>>,
    broadcaster: broadcast::Sender<String>,
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket(socket, state))
}

async fn handle_websocket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.broadcaster.subscribe();
    tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            if socket
                .send(Message::Text(Utf8Bytes::from(message)))
                .await
                .is_err()
            {
                break;
            }
        }
    });
}
async fn add_to_queue(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    if let Some(new_message) = payload.get("message").and_then(|v| v.as_str()) {
        let mut messages = state.messages.lock().await;
        let _ = state.broadcaster.send(new_message.to_string());
        
        if new_message.starts_with("add") {
            messages.push(new_message.to_string());
        }
        if new_message.starts_with("remove") {
            let data: Vec<_> = new_message.split(":").collect();
            if data.len() == 2 {
                let position = messages.iter().position(|it| it.ends_with(data[1]));
                if position != None { messages.remove(position.unwrap()); }
            }
        }
        return "Message added".into_response();
    }
    ("Invalid request").into_response()
}
async fn list_queue(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<String>> {
    Json(state.messages.lock().await.to_vec().iter().filter(|it| {
        let data: Vec<_> = it.split(":").collect();
        return data[0] == "add";
    }).cloned().collect())
}
async fn render_index() -> Html<String> {
    let path = PathBuf::from("./templates/index.html");
    match fs::read_to_string(&path).await {
        Ok(html_content) => Html(html_content),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            Html("<h1>Nie udało się załadować pliku HTML</h1>".to_string())
        }
    }
}
