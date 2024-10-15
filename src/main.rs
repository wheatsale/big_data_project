use axum::{
    extract::Form, http::StatusCode, response::IntoResponse, routing::{get, post}, Router
};
use serde::Deserialize;
use askama_axum::Template;
use std::env;

mod reddit_scraper;
pub mod models;
pub mod schema;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let app = app.route("/results", post(search));

    let app = app.fallback(handler_404);

    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => String::from("8080"),
    };

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate { }

async fn root() -> impl IntoResponse {
    IndexTemplate { }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct SearchInput {
    provider_name: String,
    care_type: String,
    subreddits: String,
}

#[derive(Template)]
#[template(path = "results.html")]
struct ResultsTemplate {
    provider: String,
    care: String,
    subreddits: String,
}

async fn search(Form(search): Form<SearchInput>) -> impl IntoResponse {
    let provider_result = format!("You searched for {}", search.provider_name);
    let care_result = format!("You searched for {}", search.care_type);
    let subreddit_results = format!("You searched for {}", search.subreddits);

    ResultsTemplate{ 
        provider: provider_result,
        care: care_result,
        subreddits: subreddit_results
    }
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "path not found")
}
