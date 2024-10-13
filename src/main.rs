use axum::{
    routing::{get, post},
    extract::Form,
    response::Html,
    Router,
};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    user_input: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async {
        Html(r#"
        <form action="/echo_user_input" method="POST">
            <input name="user_input">
            <input type="submit" value="Submit!">
        </form>
        "#)
    }))

    .route("/echo_user_input", post(accept_form));

    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => String::from("80"),
    };

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn accept_form(Form(input): Form<Input>) -> String {
    input.user_input
}
