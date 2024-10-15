use roux::{ self, Subreddit, Reddit };
use std::env;

//fn authorize() {

//}

#[derive(Debug)]
pub struct Post {
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
}

pub async fn scrape_subreddit(name: &str) -> Vec<Post> {
    let subreddit = Subreddit::new(name);
    let latest = subreddit.latest(100, None).await;

    match latest {
        Ok(posts) => {
            posts.data.children.iter().filter_map(|post| {
                let post = &post.data;
                let content = match &post.selftext_html {
                    Some(content) => content,
                    None => match &post.url {
                        Some(content) => content,
                        None => return None
                    }
                };
                
                Some(Post {
                    id: post.id.clone(),
                    title: post.title.clone(),
                    content: content.clone(),
                    permalink: post.permalink.clone(),
                    subreddit: post.subreddit.clone(),
                    author: post.author.clone(),
                    over_18: post.over_18,
                    num_comments: post.num_comments,
                    score: post.score,
                    ups: post.ups,
                    downs: post.downs,
                    created: post.created,
                })
            }).collect()
        },
        Err(_) => Vec::new(),
    }
}
