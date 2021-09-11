use super::models;
use warp::Filter;

pub fn swatch_gen() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("swatch")
        .and(warp::get())
        .and(warp::query::<models::SwatchOptions>())
        .and_then(crate::handlers::swatch_gen)
}

pub fn swatch_gen_2() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("swatch.png")
        .and(warp::get())
        .and(warp::query::<models::SwatchOptions>())
        .and_then(crate::handlers::swatch_gen)
}

#[cfg(test)]
mod tests {
    use crate::models::{self, SwatchOptions};
    use csscolorparser::Color;

    #[tokio::test]
    async fn test_swatch_option_parsing_size() {
        let filter = warp::query::<models::SwatchOptions>();
        let res = warp::test::request()
            .path("/?color=black&size=32")
            .filter(&filter)
            .await
            .unwrap();
        assert_eq!(
            res,
            SwatchOptions {
                color: Color::from_html("black").unwrap(),
                size: Some(32),
                ..SwatchOptions::default()
            }
        )
    }
}
