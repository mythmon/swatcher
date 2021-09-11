use crate::errors::Error;
use crate::models;
use warp::http::{header, HeaderValue};
use warp::reply::Response;
use warp::Rejection;

pub async fn swatch_gen(options: models::SwatchOptions) -> Result<impl warp::Reply, Rejection> {
    let mut body = Vec::new();
    crate::image::gen_swatch(&mut body, options.width(), options.height(), options.color)
        .map_err(warp::reject::custom::<Error>)?;

    let mut response = Response::new(body.into());
    let headers_mut = response.headers_mut();
    headers_mut.insert(header::CONTENT_TYPE, HeaderValue::from_static("image/png"));
    headers_mut.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("max-age=2419200,public,immutable"),
    );
    Ok(response)
}
