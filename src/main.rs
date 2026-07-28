#![allow(unused)]

use axum::{
    response::Html,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );

    // region: -- Start Server

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("->> LISTENING on http://127.0.0.1:8080");

    axum::serve(listener, routes_hello)
        .await
        .unwrap();

    // endregion: -- Start Server
}