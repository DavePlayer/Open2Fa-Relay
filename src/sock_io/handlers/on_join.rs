use socketioxide::extract::{Data, SocketRef};
use tracing::info;

pub fn handler(socket: SocketRef, data: Data<String>) {
    info!("request to join room: {}", data.to_string());
    let _ = socket.leave_all();

    let _ = socket.join(data.to_string());
}
