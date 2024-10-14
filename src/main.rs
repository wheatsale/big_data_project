use axum::{
    extract::Form, http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}, Router
};
use serde::Deserialize;
use std::{
    env, fs::File, io::Read, path::PathBuf
};

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Input {
    user_input: String,
}

// Returns an HTML response if the static file is found and a 404 response otherwise
fn static_file(path: &str) -> impl IntoResponse {
    // Use CARGO_MANIFEST_DIR if run via "cargo run" and look two directories down if run from the
    // release folder (ie. via Heroku).
    let dir = match option_env!("CARGO_MANIFEST_DIR") {
        Some(env) => env,
        None => "../.."
    };

    let path = PathBuf::from(format!("{dir}/{path}"));

    match File::open(path) {
        Ok(mut file) => {
            let mut buffer = String::new();

            match file.read_to_string(&mut buffer) {
                Ok(_) => Html(buffer).into_response(),
                Err(_) => (StatusCode::NOT_FOUND, "file not found").into_response()
            }
        },
        Err(_) => (StatusCode::NOT_FOUND, "file not found").into_response()
    }
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
    static_file("resources/static/templates/index.html")
}

async fn accept_form(Form(input): Form<Input>) -> String {
    input.user_input
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "file not found")
}
