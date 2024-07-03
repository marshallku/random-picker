use base64::{engine::general_purpose::STANDARD, Engine};

fn generate_preview_image(selected_option: String) -> String {
    format!(
        "data:image/svg+xml;base64,{}",
        STANDARD.encode(format!(
            r##"
            <svg xmlns="http://www.w3.org/2000/svg" width="400" height="200">
                <rect width="100%" height="100%" fill="#f0f0f0" />
                <text x="50%" y="50%" dominant-baseline="middle" text-anchor="middle" font-family="Arial" font-size="20" fill="#333">{}</text>
            </svg>
            "##,
            selected_option
        ))
    )
}

pub fn render_index_page(selected_option: String) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>{}</title>
            <meta property="og:title" content="{}" />
            <meta property="og:image" content="{}" />
        </head>
        <body>
            <h1>Random Picker</h1>
            <p>Selected option: {}</p>
        </body>
        </html>
    "#,
        selected_option,
        selected_option,
        generate_preview_image(selected_option.clone()),
        selected_option
    )
}
