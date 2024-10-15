use diesel::{pg::Pg, prelude::*};
use bigdecimal::BigDecimal;
use crate::schema;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = schema::posts)]
#[diesel(check_for_backend(Pg))]
pub struct Post {
    pub title: String,
    pub id: String,
    pub content: String,
    pub permalink: String,
    pub subreddit: String,
    pub author: String,
    pub over_18: bool,
    pub num_comments: BigDecimal,
    pub score: BigDecimal,
    pub ups: BigDecimal,
    pub downs: BigDecimal,
    pub created: BigDecimal,
}

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = schema::comments)]
#[diesel(check_for_backend(Pg))]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub parent_id: Option<String>,
    pub author: Option<String>,
    pub permalink: Option<String>,
    pub body_html: Option<String>,
    pub over_18: Option<bool>,
    pub score: Option<BigDecimal>,
    pub ups: Option<BigDecimal>,
    pub downs: Option<BigDecimal>,
}
