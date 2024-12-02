use socketioxide::extract::{Data, SocketRef};
use tracing::info;

use crate::models::message_in::MessageIn;

pub fn handler(_socket: SocketRef, data: Data<MessageIn>) {
    // Extract the inner data and log it
    let inner_data = data.0;
    info!("Handler triggered with raw data: {:?}", inner_data);

    // Additional logic here if needed
    info!("Received message: {:?}", inner_data);
}
