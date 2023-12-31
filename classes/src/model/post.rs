use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct PostIdentifier {
    pub post_id: String,
    pub blog_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub blog_id: String,
    pub post_id: String,
    pub author: String,
    pub title: Option<String>,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPost {
    pub author: String,
    pub title: Option<String>,
    pub content: String,
}