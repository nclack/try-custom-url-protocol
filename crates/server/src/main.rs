use anyhow::Error;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(),Error> {
    pretty_env_logger::init();
    warp::serve(
        warp::path::end().and(warp::get())//.and(warp::fs::file("crates/server/static/index.html"))
        .map(|| warp::reply::html(include_str!("../static/index.html")))
    )
        .run(([127,0,0,1],8000))
        .await;
    Ok(())
}