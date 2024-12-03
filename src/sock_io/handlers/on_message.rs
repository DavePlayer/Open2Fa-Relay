use socketioxide::extract::{Data, SocketRef};
use tracing::info;

use crate::models::message_in::Message;

pub fn handler(socket: SocketRef, Data(data): Data<Message>) {
    // Extract the inner data and log it
    info!("Received message: {:?}", data);

    // let _ = socket.within(socket.id).emit("message", &data.text);
    let _ = socket.emit("message", &data.text);
}
