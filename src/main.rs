use crate::model::ModelController;

pub use self::error::{Error, Result};
use axum::{
    extract::Query,
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let routes_all = Router::new()
        .merge(routes())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_request_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
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
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_service_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_service_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "request_uuid": uuid.to_string()
                }
            });

            println!("  ->> client_error_body: {client_error_body}");

            (*status_code, Json(client_error_body)).into_response()
        });
    println!("  ->> server log line - {uuid} - Error: {service_error:?}");

    println!();
    error_response.unwrap_or(res)
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
