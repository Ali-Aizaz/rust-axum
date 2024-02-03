use crate::model::ModelController;

pub use self::error::{Error, Result};
use axum::{
    extract::Query,
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
        .merge(routes())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_request_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("->> LISTENING on {:?}\n", listener);

    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

fn routes() -> Router {
    Router::new().route("/hello", get(handler_hello))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn main_request_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!("");
    res
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!(
        "->> LISTENING {:<12} - handler_hello - {params:?}",
        "HANDLER"
    );

    let name = params.name.as_deref().unwrap_or("World");

    Html(format!("Hello <strong>{name}</strong>!"))
}
