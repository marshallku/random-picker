#[cfg(test)]
mod tests {

    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::routes::index::get_index;

    #[tokio::test]
    async fn should_not_panic_without_options() {
        let app: Router = Router::new().route("/", get(get_index));
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn should_parse_options() {
        let options: Vec<&str> = vec![
            "some_random_option1",
            "some_random_option2",
            "some_random_option3",
            "some_random_option4",
            "some_random_option5",
        ];
        let app: Router = Router::new().route("/", get(get_index));
        let response = app
            .oneshot(
                Request::builder()
                    .uri(format!("/?options={}", options.join(",")).as_str())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = String::from_utf8(
            response
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .to_vec(),
        )
        .unwrap();

        assert!(
            options.iter().any(|option| body.contains(option)),
            "Body should include one of the options"
        );
    }
}
