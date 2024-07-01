use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;

use crate::utils::random::pick_random;

#[derive(Deserialize)]
pub struct IndexParams {
    options: Option<String>,
}

pub async fn get_index(query: Query<IndexParams>) -> impl IntoResponse {
    if query.options.is_none() {
        return "No options provided".into_response();
    }

    let picked = pick_random(
        &query
            .options
            .as_ref()
            .unwrap()
            .split(',')
            .collect::<Vec<&str>>(),
    )
    .to_string();

    picked.into_response()
}
