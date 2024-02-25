use askama::Template;
use axum::response::Html;

use crate::entities::errors::AppResult;

#[derive(Template)]
#[template(path = "base.html")]
struct HelloTemplate {}

pub async fn get_hello() -> AppResult<Html<String>> {
    let data = HelloTemplate {}.render().unwrap();
    println!("Echo handler");
    Ok(Html(data))
}

pub async fn default() -> AppResult<Html<String>> {
    println!("Default");
    Ok(Html(String::from("Hello")))
}
