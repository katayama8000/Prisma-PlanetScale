#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routers_hello = Router::new().route("/hello", get(handler_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routers_hello.into_make_service())
        .await
        .unwrap()
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello() -> impl IntoResponse {
    println!("handler hello");
    Html("Hello World")
}

// async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
//     println!("handler hello - {params:?}");
//     let name = params.name.as_deref().unwrap_or("world");
//     Html(format!("Hello<strong>{name}</strong>"));
//     Html("Hello<strong>World</strong>");
// }
