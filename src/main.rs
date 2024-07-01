mod routes;
mod utils;

use axum::{routing::get, Router};
use dotenv::dotenv;
use log::info;
use routes::index::get_index;
use tokio;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    dotenv().ok();

    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "48092".to_string());
    let full_address = format!("{}:{}", address, port);
    let listener = tokio::net::TcpListener::bind(full_address.as_str())
        .await
        .unwrap();
    let app = Router::new().route("/", get(get_index));

    info!("Server running at http://{}", full_address);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
