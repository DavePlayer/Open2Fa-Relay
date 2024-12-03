use std::sync::Arc;

use axum::extract::{self, State};
use serde::Deserialize;
use socketioxide::{socket::Sid, SocketIo};
use tracing::info;

#[derive(Deserialize)]
pub struct BodyJson {
    code: String,
    #[serde(rename = "roomId")]
    room_id: Sid,
}

pub async fn handle_send_code_route(
    State(io): State<Arc<SocketIo>>,
    extract::Json(body): extract::Json<BodyJson>,
) -> &'static str {
    info!("testing websocket emit");
    let _ = io
        .within(body.room_id)
        .emit("message", &format!("sending code: {}", body.code));

    "Message Emited"
}
