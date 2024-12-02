use std::sync::Arc;

use axum::routing::get;
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;

pub mod models;
pub mod sock_io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set default tracing formating from FmtSubscriber library
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", sock_io::on_connect);
    let io_arc = Arc::new(io);

    let app = axum::Router::new()
        .route(
            "/",
            get(|| async move {
                tracing::info!("Received HTTP request");

                // Emit a message to connected clients via SocketIo
                let io = io_arc.clone();
                let _ = io.emit("message", "HTTP Request message");

                // Return the HTTP response
                "Hello. Yes I work"
            }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
