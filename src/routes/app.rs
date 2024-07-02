use axum::{routing::get, Router};

use super::index::get_index;

pub fn app() -> Router {
    Router::new().route("/", get(get_index))
}
