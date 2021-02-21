use hyper::{Body, Request, Response, StatusCode};
use routes::*;

#[routes(get "hello$")]
async fn hello() -> anyhow::Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("hello world"))?)
}

#[routes(get "world$")]
async fn world() -> anyhow::Result<Response<Body>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("world"))?)
}

routes_group!(pub root "root" {
    "h" => world,
    "w" => {"v" => {hello,world}}});

#[tokio::main]
async fn main() {
    let builder = hyper::Server::bind(&"127.0.0.1:4000".parse().unwrap());

    serve(&root::endpoint, builder).await.unwrap();
}
