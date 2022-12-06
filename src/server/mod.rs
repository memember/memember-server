/// API handler methods.
pub mod handlers;

/// API routes.
mod routes;

use crate::error::Result;
use axum::Server;
use std::net::SocketAddr;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

/// Initializes the routes, layers (middleware) and binds the server to the provided address.
pub(crate) async fn start(socket_addr: SocketAddr) -> Result<()> {
    // Initialize the application.
    let app = routes::get_api_router()
        .layer(CorsLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    // Create a server.
    Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
