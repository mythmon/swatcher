use thiserror::Error;
use warp::reject::Reject;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An error while encoding the image")]
    Image(#[from] png::EncodingError),
}

impl Reject for Error {}
