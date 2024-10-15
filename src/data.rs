use crate::models::{Post, Comment};
use diesel::{self, PgConnection, RunQueryDsl, SelectableHelper};
use bigdecimal::BigDecimal;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct RawComment {
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

pub fn create_post(
    connenction: &mut PgConnection,
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
        .get_result(connenction)
}

pub fn create_comment(
    connenction: &mut PgConnection,
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
        .get_result(connenction)
}
