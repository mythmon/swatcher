use thiserror::Error;
use warp::reject::Reject;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An error while encoding the image")]
    Encoding(#[from] png::EncodingError),
    #[error("An unexpected color format was passed")]
    ColorFormat,
}

impl Reject for Error {}
