use std::sync::Arc;

use axum::routing::{get, post};
use routes::{send_code::handle_send_code_route, test_socket_emit::handle_test_socket_emit_route};
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;

pub mod models;
pub mod routes;
pub mod sock_io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set default tracing formating from FmtSubscriber library
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", sock_io::on_connect);
    let io_arc = Arc::new(io);

    let app = axum::Router::new()
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                // .layer(axum::middleware::from_fn(jwt_verify_middleware))
                .layer(layer),
        )
        .route("/", get(handle_test_socket_emit_route))
        .route("/sendCode", post(handle_send_code_route));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, axum::Router::with_state(app, io_arc)).await?;

    Ok(())
}

// if JWT verification would be important in the future
// NEED TO INSTALL JSONWEBTOKEN PACKAGE
// async fn jwt_verify_middleware(
//     req: axum::http::Request<Body>,
//     next: axum::middleware::Next,
// ) -> impl IntoResponse {
//     // Extract headers
//     let headers = req.headers().clone();
//     let authorization = headers.get("Authorization").cloned();
//     dotenv::dotenv().ok();

//     // You can pass this data to your socket.io logic
//     if let Some(auth) = authorization {
//         let (token_type, token) = auth
//             .to_str()
//             .unwrap_or("")
//             .split_once(' ')
//             .unwrap_or(("", ""));

//         if token_type == "bearer" {
//             println!("got a token: {}", token);
//             let secret_str = std::env::var("JWTSECRET").unwrap_or(String::from(""));
//             info!("secret string: {}", secret_str);
//             match decode::<serde_json::Value>(
//                 token,
//                 &DecodingKey::from_secret(secret_str.as_ref()),
//                 &Validation::default(),
//             ) {
//                 Ok(claims) => {
//                     info!("claims: {:?}", claims);
//                 }
//                 Err(err) => {
//                     error!("Err: {:?}", err);
//                     return (
//                         StatusCode::FORBIDDEN,
//                         "token decoding failed. invalid token",
//                     )
//                         .into_response();
//                 }
//             }
//         } else {
//             return (StatusCode::BAD_REQUEST, "invalid header type").into_response();
//         }
//     } else {
//         return (StatusCode::BAD_REQUEST, "No Authentication header provided").into_response();
//     }

//     next.run(req).await
// }
