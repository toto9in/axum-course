#![allow(unused)]

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("<h1>Hello, World!</h1>".to_string()) }),
    );

    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    println!("SERVER LISTENING ON {:?}\n", listener.local_addr());
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> -{:<12} - handler_hello - {params:?}", "HANDLER");
    Html("<h1>Hello, World!</h1>".to_string())
}
