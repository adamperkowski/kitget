//! Error handling module

use thiserror::Error as ThisError;

/// Custom error type specification for kitget
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("image loading failed: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("image display failed: {0}")]
    DisplayError(#[from] viuer::ViuError),

    #[error("request failed: {0}")]
    Request(String),

    #[error("animated kittehz don't support tagz")]
    GIFsNoTags,
    #[error("monochrome kittehz don't support colors (duh)")]
    MonoColors,
    #[error("blur valuez must between 0.3 and 1000")]
    BlurValue,
    #[error("RGB colorz must be together")]
    RGBColors,

    #[error("kitteh not found")]
    NotFound,
}

impl Error {
    /// Pretty-print the error message
    pub fn pretty(&self) {
        println!("😿 {self} 😿");
    }
}

/// Result type alias for kitget
pub type Result<T> = std::result::Result<T, Error>;
