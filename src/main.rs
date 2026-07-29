#![allow(unused)]

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("->> LISTENING on http://127.0.0.1:8080");

    axum::serve(listener, routes_hello)
        .await
        .unwrap();
}


async fn handler_hello() -> impl IntoResponse {
    println!("--> {:<12} - handler_hello", "HANDLER");

    Html("Hello <strong>World!!!</strong>")
}

