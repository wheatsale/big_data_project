use axum::{
    extract::{Form, Json}, http::StatusCode, response::IntoResponse, routing::{get, post}, Router
};
use serde::Deserialize;
use askama_axum::Template;
use std::env;
use data::RawPost;
use diesel::prelude::*;

pub mod models;
pub mod schema;
pub mod data;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let app = app.route("/results", post(search));
    let app = app.route("/posts", post(insert_posts));

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
}

#[derive(Template)]
#[template(path = "results.html")]
struct ResultsTemplate {
    provider: String,
    care: String,
    subreddits: String,
}

async fn search(Form(search): Form<SearchInput>) -> impl IntoResponse {
    use self::schema::posts::dsl::*;
    use self::models::Post;

    let provider_result = format!("You searched for {}", search.provider_name);
    let care_result = format!("You searched for {}", search.care_type);
    
    let connection = &mut data::establish_connection();
    let results = posts
            .limit(5)
            .select(Post::as_select())
            .load(connection)
            .expect("Error loading posts");

    let mut subreddit_results = String::new();

    for post in results {
        subreddit_results = format!("{subreddit_results} - {}", post.title);
    }

    ResultsTemplate{ 
        provider: provider_result,
        care: care_result,
        subreddits: subreddit_results
    }
}

async fn insert_posts(Json(payload): Json<Vec<RawPost>>) {
    data::insert_posts(payload).await.unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "path not found")
}
