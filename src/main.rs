use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>world</strong>!") }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("->> LISTENING on {:?}\n", listener);

    axum::serve(listener, routes_hello).await.unwrap();
}
