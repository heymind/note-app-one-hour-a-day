use anyhow::Result;
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server, StatusCode,
};
use std::convert::Infallible;
use std::net::SocketAddr;
#[tokio::main]
async fn main() -> Result<()> {
    let listen_addr: SocketAddr = "127.0.0.1:4000".parse()?;

    Server::bind(&listen_addr)
        .serve(make_service_fn(|_socket: &AddrStream| async move {
            Ok::<_, Infallible>(service_fn(|_req: Request<Body>| async {
                Ok::<_, Infallible>(
                    Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from("hello world"))
                        .unwrap(),
                )
            }))
        }))
        .await?;
    Ok(())
}
