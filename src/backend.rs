use dioxus::prelude::*;
use std::fs;

const BLOG_DIR: &str = "/var/lib/blog";

#[get("/api/blog/:id")]
pub async fn get_blog(id: i32) -> Result<String> {
    let file_path = format!("{}/{}.md", BLOG_DIR, id);
    match fs::read_to_string(&file_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e.into()),
    }
}
