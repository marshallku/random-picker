use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IndexParams {
    text: Option<String>,
}

pub async fn get(query: Query<IndexParams>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "image/svg+xml".parse().unwrap());

    let text = query
        .text
        .as_ref()
        .unwrap_or(&String::from("No options provided"))
        .to_string();

    let image = format!(
        r##"
        <svg xmlns="http://www.w3.org/2000/svg" width="400" height="200">
            <rect width="100%" height="100%" fill="#f0f0f0" />
            <text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" font-family="Arial" font-size="20" fill="#333">{}</text>
        </svg>
        "##,
        text
    );

    (StatusCode::OK, headers, image)
}
