use crate::types::{photo_size::PhotoSize, primitive::Integer};
use serde::Deserialize;

/// Video file
#[derive(Clone, Debug, Deserialize)]
pub struct Video {
    /// Unique identifier for this file
    pub file_id: String,
    /// Video width as defined by sender
    pub width: Integer,
    /// Video height as defined by sender
    pub height: Integer,
    /// Duration of the video in seconds as defined by sender
    pub duration: Integer,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// Mime type of a file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
}
