CREATE TABLE posts (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    permalink TEXT NOT NULL,
    subreddit TEXT NOT NULL,
    author TEXT NOT NULL,
    over_18 BOOLEAN NOT NULL,
    num_comments NUMERIC NOT NULL,
    score NUMERIC NOT NULL,
    ups NUMERIC NOT NULL,
    downs NUMERIC NOT NULL,
    created NUMERIC NOT NULL
);

CREATE TABLE comments (
    id TEXT PRIMARY KEY,
    post_id TEXT NOT NULL,
    parent_id TEXT,
    author TEXT,
    permalink TEXT,
    body_html TEXT,
    over_18 BOOLEAN,
    score NUMERIC,
    ups NUMERIC,
    downs NUMERIC
);
