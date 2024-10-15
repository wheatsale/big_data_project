// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Text,
        post_id -> Text,
        parent_id -> Nullable<Text>,
        author -> Nullable<Text>,
        permalink -> Nullable<Text>,
        body_html -> Nullable<Text>,
        over_18 -> Nullable<Bool>,
        score -> Nullable<Numeric>,
        ups -> Nullable<Numeric>,
        downs -> Nullable<Numeric>,
    }
}

diesel::table! {
    posts (id) {
        id -> Text,
        title -> Text,
        content -> Text,
        permalink -> Text,
        subreddit -> Text,
        author -> Text,
        over_18 -> Bool,
        num_comments -> Numeric,
        score -> Numeric,
        ups -> Numeric,
        downs -> Numeric,
        created -> Numeric,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    posts,
);
