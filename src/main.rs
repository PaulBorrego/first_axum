#![allow(unused)] // only in beginning

use axum::{
    Router,
    routing::get,
    response::Html,
};


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new().route(
            "/hello",
            get(|| async { Html("<h1>Hello, World!</h1>")})
    );

    // start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}