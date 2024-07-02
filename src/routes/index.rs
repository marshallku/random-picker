use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;

use crate::{pages::index::render_index_page, utils::random::pick_random};

#[derive(Deserialize)]
pub struct IndexParams {
    options: Option<String>,
}

pub async fn get_index(query: Query<IndexParams>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "text/html".parse().unwrap());

    let picked = pick_random(
        &query
            .options
            .as_ref()
            .unwrap_or(&String::from("No options provided"))
            .split(',')
            .collect::<Vec<&str>>(),
    )
    .to_string();

    (StatusCode::OK, headers, render_index_page(picked))
}
