use warp::Filter;

mod errors;
mod filters;
mod handlers;
mod image;
mod models;

#[tokio::main]
async fn main() {
    let swatch_gen = filters::swatch_gen();
    let swatch_gen_png = filters::swatch_gen_2();
    let index = warp::path::end().map(|| "Swatcher");
    let routes = warp::get().and(swatch_gen.or(swatch_gen_png).or(index));
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
