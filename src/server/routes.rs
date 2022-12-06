use crate::server::handlers::*;
use axum::routing::get;
use axum::Router;

/// Returns the API routes.
///
/// Routes should be defined here with their appropriate HTTP methods.
///
/// ```ignore
/// Router::new()
///     // `GET /` goes to `root`
///     .route("/", get(root))
///     // `POST /users` goes to `create_user`
///     .route("/users", post(create_user));
///```
///
/// reference: <https://docs.rs/axum/latest/axum/routing/index.html>
pub(crate) fn get_api_router() -> Router {
    Router::new().route("/health", get(health))
}
