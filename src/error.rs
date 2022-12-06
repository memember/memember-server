use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error as ThisError;

/// Custom error implementation.
#[derive(ThisError, Debug)]
pub enum Error {
    /// Error that may occur during API requests.
    #[error("API error ({}): `{}`", 0.1, 0.0)]
    Api((StatusCode, Json<&'static str>)),

    /// Error that may occur during I/O operations.
    #[error("IO error: `{0}`")]
    Io(#[from] std::io::Error),

    /// Error that may occur while setting up the logger.
    #[error("Could not set up global logger: `{0}`")]
    Logger(#[from] tracing::dispatcher::SetGlobalDefaultError),

    /// Error that may occur during HTTP streams.
    #[error("HTTP error: `{0}`")]
    Http(#[from] hyper::Error),

    /// Error that may occur while parsing a IP address or a socket address.
    #[error("Failed to parse IP address: `{0}`")]
    AddrParse(#[from] std::net::AddrParseError),

    /// Error that may occur while parsing integers.
    #[error("Failed to parse integer: `{0}`")]
    IntParse(#[from] std::num::ParseIntError),

    /// Error that may occur while interacting with environment variables.
    #[error("Environment variable error: `{0}`")]
    Var(#[from] std::env::VarError),
}

/// Response implementation for error.
///
/// This is necessary for returning an [`Error`] from a handler.
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #[allow(clippy::match_single_binding)]
        let status_code = match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let response = (status_code, Json(self.to_string())).into_response();
        tracing::error!("{:?}", response);
        response
    }
}

/// Type alias for the standard [`Result`] type.
///
/// It makes it easier to return `Result`s from functions
/// without specifying the error type. It uses the custom
/// defined error type as default. See [`Error`].
///
/// ```ignore
/// fn example() -> Result<()> {
///   // propagate the errors with `?` operator
///   std::fs::read_to_string("example.txt")?;
///   Ok(())
/// }
/// ```
///
/// reference: <https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html>
///
/// [`Result`]: std::result::Result
/// [`Error`]: crate::error::Error
pub(crate) type Result<T> = std::result::Result<T, Error>;
