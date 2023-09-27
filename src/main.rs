#![allow(unused)]

use std::net::SocketAddr;

use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routers_hello = Router::new().route(
        "/hello",
        get(|| async { Html("hello<strong>world</strong>") }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> listening on {addr}");
    axum::Server::bind(&addr)
        .serve(routers_hello.into_make_service())
        .await
        .unwrap()
}
