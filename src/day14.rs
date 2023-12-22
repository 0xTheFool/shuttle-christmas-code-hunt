use askama::Template;
use axum::{debug_handler, response::Html, Json};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    body: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Body {
    content: String,
}

#[debug_handler]
pub async fn render_html_unsafe(Json(data): Json<Body>) -> Html<&'static str> {
    let html = format!(
        "<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>",
        data.content
    );
    Html(html.leak())
}

#[debug_handler]
pub async fn render_html_safe(Json(data): Json<Body>) -> Html<&'static str> {
    let template = BaseTemplate {
        body: &data.content,
    };
    let reply_html = template.render().unwrap();
    Html(reply_html.leak())
}
