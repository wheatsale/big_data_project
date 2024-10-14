use axum::{
    extract::Form, http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, Router
};
use serde::Deserialize;
use std::{
    env, fs::File, io::{self, Read}, path::PathBuf
};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    user_input: String,
}

// Returns a string from a static file in the project directory
fn static_file(path: &str) -> Result<String, io::Error> {
    // Use CARGO_MANIFEST_DIR if run via "cargo run" and look two directories down if run from the
    // release folder (ie. via Heroku).
    let dir = match option_env!("CARGO_MANIFEST_DIR") {
        Some(env) => env,
        None => "app"
    };

    let path = PathBuf::from(format!("{dir}/{path}"));

    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;
    Ok(buffer)
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
    match static_file("resources/static/templates/index.html") {
        Ok(content) => Html(content).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "file not found").into_response()
    }
}

async fn accept_form(Form(input): Form<Input>) -> String {
    input.user_input
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "file not found")
}
