use serde::{Deserialize, Serialize};

/// Image data.
#[derive(Default, Deserialize, Serialize)]
pub struct Image {
    /// Raw image data (RGBA8).
    pub data: Vec<u8>,
    /// Image text (extracted via OCR).
    pub text: Option<String>,
    /// Original image URL.
    pub url: String,
    /// Image hint.
    pub hint: Option<String>,
}
