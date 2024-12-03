use socketioxide::extract::SocketRef;
use tracing::info;

pub mod handlers;
pub mod store;

pub fn on_connect(socket: SocketRef) {
    info!("new user connected: {}", socket.id);
    let _ = socket.emit("message", &format!("connId|{}", socket.id));
    let _ = socket.leave_all();
    let _ = socket.join(socket.id);

    socket.on("message", handlers::on_message::handler);
    info!("Message handler registered");

    socket.on("join", handlers::on_join::handler);
    info!("Join handler registered");
}
