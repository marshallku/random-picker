mod pages;
mod routes;
mod utils;

use dotenv::dotenv;
use log::info;
use routes::app::app;
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
    let app = app();

    info!("Server running at http://{}", full_address);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
