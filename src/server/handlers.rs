use crate::error::{Error, Result};
use crate::image::Image;
use axum::extract::{Json, Multipart};
use axum::http::StatusCode;
use axum::response::IntoResponse;

/// Returns 200 and "OK" message.
pub async fn health() -> Result<impl IntoResponse> {
    let response = "OK";
    Ok((StatusCode::OK, Json(response)))
}

/// Accepts an image and returns the processed data.
pub async fn save_image(mut multipart: Multipart) -> Result<Json<Image>> {
    let mut image = Image::default();
    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("image") => {
                let data = field.bytes().await?;
                let loaded_image = image::load_from_memory(&data)?;
                let rgba8 = loaded_image.as_rgba8().ok_or_else(|| {
                    Error::Api((
                        StatusCode::BAD_REQUEST,
                        Json("failed to convert image to RGBA8"),
                    ))
                })?;
                image.data = rgba8.to_vec();
                match tesseract::ocr_from_frame(
                    rgba8.as_raw(),
                    rgba8.width().try_into()?,
                    rgba8.height().try_into()?,
                    4,
                    i32::try_from(rgba8.width())? * 4,
                    "eng",
                ) {
                    Ok(text) => {
                        image.text = Some(text);
                    }
                    Err(e) => {
                        tracing::error!("OCR failed: {:?}", e)
                    }
                }
            }
            Some("url") => {
                image.url = String::from_utf8_lossy(&field.bytes().await?).to_string();
            }
            Some("hint") => {
                image.hint = Some(String::from_utf8_lossy(&field.bytes().await?).to_string());
            }
            _ => {}
        }
    }
    Ok(Json(image))
}
