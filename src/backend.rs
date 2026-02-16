use dioxus::prelude::*;

use crate::models::{BlogData, HeaderLink};

const BLOG_DIR: &str = "./blog";

#[server]
pub async fn get_blog_data(id: usize) -> Result<BlogData, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::utils::CustomHighlighter;
        use comrak::{Options, markdown_to_html_with_plugins, options::Plugins};
        use scraper::{Html, Selector};
        use tokio::fs;

        let file_path = format!("{}/{}.md", BLOG_DIR, id);
        let content = fs::read_to_string(&file_path)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to read post: {}", e)))?;

        let adapter = CustomHighlighter::new("base16-eighties.dark");

        let mut options = Options::default();
        options.extension.strikethrough = true;
        options.extension.tagfilter = true;
        options.extension.table = true;
        options.extension.autolink = true;
        options.extension.header_ids = Some("".to_string());

        let mut plugins = Plugins::default();
        plugins.render.codefence_syntax_highlighter = Some(&adapter);

        let html_content = markdown_to_html_with_plugins(&content, &options, &plugins);

        let document = Html::parse_fragment(&html_content);
        let header_selector = Selector::parse("h2, h3").unwrap();
        let anchor_selector = Selector::parse("a[id]").unwrap();

        let headers: Vec<HeaderLink> = document
            .select(&header_selector)
            .filter_map(|element| {
                if let Some(anchor) = element.select(&anchor_selector).next() {
                    let id = anchor.value().attr("id").unwrap().to_string();
                    let title = element.text().collect::<Vec<_>>().join("");
                    return Some(HeaderLink { id, title });
                }
                None
            })
            .collect();

        Ok(BlogData {
            content: html_content,
            headers,
        })
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
