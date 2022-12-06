//! An example web server template using axum framework.

// Custom lint rules for compilation/clippy.
//
// - missing_docs: abort the compilation if there are items with missing documentation.
//   - reference:  https://doc.rust-lang.org/rustdoc/lints.html
//
// - clippy::unwrap_used: deny the .unwrap() calls on `Option`s and on `Result`s.
//   - explanation:       prevents the thread from panicking on `Err` values
//   - reference:         https://rust-lang.github.io/rust-clippy/master
#![deny(missing_docs, clippy::unwrap_used)]

/// Error implementation.
mod error;

/// HTTP server implementation.
pub mod server;

use crate::error::Result;
use std::env;
use std::net::SocketAddr;

/// Default value for the bind address.
pub(crate) const DEFAULT_SOCKET_ADDR: &str = "0.0.0.0";
/// Default value for the port.
pub(crate) const DEFAULT_SOCKET_PORT: &str = "8080";
/// Name of the environment variable for bind address.
pub(crate) const SOCKET_ADDR_ENV: &str = "ADDR";
/// Name of the environment variable for port.
pub(crate) const SOCKET_PORT_ENV: &str = "PORT";

/// Returns the socket address built from environment variables.
///
/// See [`SOCKET_ADDR_ENV`] and [`SOCKET_PORT_ENV`]
fn get_socket_addr() -> Result<SocketAddr> {
    Ok(SocketAddr::new(
        env::var(SOCKET_ADDR_ENV)
            .unwrap_or_else(|_| DEFAULT_SOCKET_ADDR.to_string())
            .parse()?,
        env::var(SOCKET_PORT_ENV)
            .unwrap_or_else(|_| DEFAULT_SOCKET_PORT.to_string())
            .parse()?,
    ))
}

/// Starts the server (blocking).
pub fn start_server() -> Result<()> {
    // Initialize the logger.
    tracing_subscriber::fmt::init();

    // Get the socket address.
    let socket_addr = get_socket_addr()?;

    // Start the server via tokio runtime.
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            tracing::info!("Starting server at {}", socket_addr);
            server::start(socket_addr).await
        })?;

    Ok(())
}
