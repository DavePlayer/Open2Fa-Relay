use std::sync::Arc;

use axum::extract::State;
use socketioxide::SocketIo;
use tracing::info;

pub async fn handle_test_socket_emit_route(State(io): State<Arc<SocketIo>>) -> &'static str {
    info!("testing websocket emit");
    let _ = io.emit("message", "Http Request Message");

    "Hello from /"
}
