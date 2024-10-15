use crate::models::{Post, Comment};
use diesel::{self, PgConnection, Connection, RunQueryDsl, SelectableHelper};
use bigdecimal::{BigDecimal, FromPrimitive};
use serde::{Serialize, Deserialize};
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct RawPost {
    id: String,
    title: String,
    content: String,
    permalink: String,
    subreddit: String,
    author: String,
    over_18: bool,
    num_comments: u64,
    score: f64,
    ups: f64,
    downs: f64,
    created: f64,
    comments: Vec<RawComment>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawComment {
    id: String,
    post_id: String,
    parent_id: Option<String>,
    author: Option<String>,
    permalink: Option<String>,
    body_html: Option<String>,
    over_18: Option<bool>,
    score: Option<i32>,
    ups: Option<i32>,
    downs: Option<i32>,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn insert_posts(posts: Vec<RawPost>) -> Result<(), diesel::result::Error> {
    let mut connection = establish_connection();

    for post in posts {
        create_post(
            &mut connection,
            post.title,
            post.id,
            post.content,
            post.permalink,
            post.subreddit,
            post.author,
            post.over_18,
            BigDecimal::from_u64(post.num_comments).unwrap(),
            BigDecimal::from_f64(post.score).unwrap(),
            BigDecimal::from_f64(post.ups).unwrap(),
            BigDecimal::from_f64(post.downs).unwrap(),
            BigDecimal::from_f64(post.created).unwrap()
        ).await?;

        for comment in post.comments {
            create_comment(
                &mut connection,
                comment.id,
                comment.post_id,
                comment.parent_id,
                comment.author,
                comment.permalink,
                comment.body_html,
                comment.over_18,
                match comment.score {
                    Some(score) => Some(BigDecimal::from_i32(score).unwrap()),
                    None => None
                },
                match comment.ups {
                    Some(ups) => Some(BigDecimal::from_i32(ups).unwrap()),
                    None => None
                },
                match comment.downs {
                    Some(downs) => Some(BigDecimal::from_i32(downs).unwrap()),
                    None => None
                },
            ).await?;
        }
    }

    Ok(())
}

pub async fn create_post(
    connection: &mut PgConnection,
    title: String,
    id: String,
    content: String,
    permalink: String,
    subreddit: String,
    author: String,
    over_18: bool,
    num_comments: BigDecimal,
    score: BigDecimal,
    ups: BigDecimal,
    downs: BigDecimal,
    created: BigDecimal,
) -> Result<Post, diesel::result::Error> {
    use crate::schema::posts;

    let post = Post {
        title,
        id,
        content,
        permalink,
        subreddit,
        author,
        over_18,
        num_comments,
        score,
        ups,
        downs,
        created,
    };

    diesel::insert_into(posts::table)
        .values(post)
        .returning(Post::as_returning())
        .get_result(connection)
}

pub async fn create_comment(
    connection: &mut PgConnection,
    id: String,
    post_id: String,
    parent_id: Option<String>,
    author: Option<String>,
    permalink: Option<String>,
    body_html: Option<String>,
    over_18: Option<bool>,
    score: Option<BigDecimal>,
    ups: Option<BigDecimal>,
    downs: Option<BigDecimal>,
) -> Result<Comment, diesel::result::Error> {
    use crate::schema::comments;

    let comment = Comment {
        id,
        post_id,
        parent_id,
        author,
        permalink,
        body_html,
        over_18,
        score,
        ups,
        downs,
    };

    diesel::insert_into(comments::table)
        .values(comment)
        .returning(Comment::as_returning())
        .get_result(connection)
}
