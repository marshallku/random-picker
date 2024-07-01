use axum::response::IntoResponse;

pub async fn get_index() -> impl IntoResponse {
    "Hello World!".into_response()
}
