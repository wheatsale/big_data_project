use axum::{
    extract::Form, http::StatusCode, response::IntoResponse, routing::{get, post}, Router
};
use serde::Deserialize;
use askama_axum::Template;
use std::env;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    user_input: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    test: &'a str,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let app = app.route("/echo_user_input", post(accept_form));

    let app = app.fallback(handler_404);

    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => String::from("80"),
    };

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    IndexTemplate { test: "bleh" }
}

async fn accept_form(Form(input): Form<Input>) -> String {
    input.user_input
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "path not found")
}
