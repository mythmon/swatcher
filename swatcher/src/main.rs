mod errors;
mod filters;
mod handlers;
mod image;
mod models;

#[tokio::main]
async fn main() {
    let handler = filters::swatch_gen();
    warp::serve(handler).run(([127, 0, 0, 1], 3030)).await;
}
