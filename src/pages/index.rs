pub fn render_index_page(selected_option: String) -> String {
    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Random Picker</title>
        </head>
        <body>
            <h1>Random Picker</h1>
            <p>Selected option: {}</p>
        </body>
        </html>
    "#,
        selected_option
    )
}
