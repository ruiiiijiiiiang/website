use dioxus::prelude::*;
use std::{ffi::OsStr, fs, path::Path};

const BLOG_DIR: &str = "./blog";

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

#[get("/api/blog/count")]
pub async fn get_blog_count() -> Result<usize> {
    let mut count = 0;
    for entry in fs::read_dir(BLOG_DIR)? {
        let entry = entry?;
        if entry.metadata()?.is_file()
            && get_extension_from_filename(entry.file_name().to_str().unwrap()) == Some("md")
        {
            count += 1;
        }
    }
    Ok(count)
}

#[get("/api/blog/:id")]
pub async fn get_blog(id: usize) -> Result<String> {
    let file_path = format!("{}/{}.md", BLOG_DIR, id);
    match fs::read_to_string(&file_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(e.into()),
    }
}
