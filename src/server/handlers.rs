use crate::error::Result;
use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Returns 200 and "OK" message.
pub async fn health() -> Result<impl IntoResponse> {
    let response = "OK";
    Ok((StatusCode::OK, Json(response)))
}
