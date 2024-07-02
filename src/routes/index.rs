use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

use crate::utils::random::pick_random;

#[derive(Deserialize)]
pub struct IndexParams {
    options: Option<String>,
}

pub async fn get_index(query: Query<IndexParams>) -> impl IntoResponse {
    let picked = pick_random(
        &query
            .options
            .as_ref()
            .unwrap_or(&String::from("No options provided"))
            .split(',')
            .collect::<Vec<&str>>(),
    )
    .to_string();

    picked.into_response()
}
