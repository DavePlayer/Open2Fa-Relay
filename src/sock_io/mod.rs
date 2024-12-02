use socketioxide::extract::SocketRef;
use tracing::info;

pub mod handlers;

pub fn on_connect(socket: SocketRef) {
    info!("new user connected: {}", socket.id);

    socket.on("message", handlers::on_message::handler);
    info!("Message handler registered");

    socket.on("join", handlers::on_join::handler);
    info!("Join handler registered");
}
