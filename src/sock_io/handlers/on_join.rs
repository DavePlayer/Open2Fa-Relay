use socketioxide::extract::{Data, SocketRef};
use tracing::info;

pub fn handler(socket: SocketRef, Data(data): Data<String>) {
    info!("request to join room: {}", data);
    let _ = socket.leave_all();

    let _ = socket.join(data);
}
