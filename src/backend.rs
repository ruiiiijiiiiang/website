use dioxus::prelude::*;

const BLOG_DIR: &str = "./blog";

#[server]
pub async fn get_blog_content(id: usize) -> Result<String, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use pulldown_cmark::{Options, Parser, html};
        use tokio::fs;

        let file_path = format!("{}/{}.md", BLOG_DIR, id);
        let content = fs::read_to_string(&file_path)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to read post: {}", e)))?;

        let mut options = Options::empty();
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
        let parser = Parser::new_ext(&content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        Ok(html_output)
    }
    #[cfg(not(feature = "server"))]
    Err(ServerFnError::new("Server only"))
}

#[server]
pub async fn get_blog_count() -> Result<usize> {
    #[cfg(feature = "server")]
    {
        use std::{ffi::OsStr, fs, path::Path};

        let mut count = 0;
        for entry in fs::read_dir(BLOG_DIR)? {
            let entry = entry?;
            if entry.metadata()?.is_file() {
                let filename = entry.file_name();
                let filename = filename.to_str().unwrap();
                if Path::new(filename).extension().and_then(OsStr::to_str) == Some("md") {
                    count += 1;
                }
            }
        }
        Ok(count)
    }
    #[cfg(not(feature = "server"))]
    Err(ServerFnError::new("Server only"))
}
