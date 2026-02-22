use dioxus::prelude::*;

use crate::models::{BlogData, BlogLink, BlogMeta, HeaderLink};

const BLOG_DIR: &str = "./blog";

#[cfg(feature = "server")]
use tokio::sync::OnceCell;
#[cfg(feature = "server")]
static POST_CACHE: OnceCell<Vec<(String, BlogMeta)>> = OnceCell::const_new();

#[server]
pub async fn get_blog_data(slug: String) -> Result<BlogData, ServerFnError> {
    use comrak::{Options, markdown_to_html_with_plugins, options::Plugins};
    use gray_matter::{Matter, engine::YAML};
    use scraper::{Html, Selector};
    use tokio::fs;

    use crate::utils::CustomHighlighter;

    let is_valid_slug = slug
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if !is_valid_slug || slug.is_empty() {
        return Err(ServerFnError::new("Invalid post slug format"));
    }

    let sorted_posts = get_cached_posts().await?;

    let current_index = sorted_posts
        .iter()
        .position(|(s, _)| s == &slug)
        .ok_or_else(|| ServerFnError::new("Post not found"))?;

    let next_post = if current_index > 0 {
        let (n_slug, n_meta) = &sorted_posts[current_index - 1];
        Some(BlogLink {
            slug: n_slug.clone(),
            title: n_meta.title.clone(),
        })
    } else {
        None
    };

    let prev_post = if current_index < sorted_posts.len() - 1 {
        let (p_slug, p_meta) = &sorted_posts[current_index + 1];
        Some(BlogLink {
            slug: p_slug.clone(),
            title: p_meta.title.clone(),
        })
    } else {
        None
    };

    let file_path = format!("{}/{}.md", BLOG_DIR, slug);
    let file_content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to read post: {}", e)))?;

    let adapter = CustomHighlighter::new("base16-eighties.dark");

    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.front_matter_delimiter = Some("---".to_string());

    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let content = markdown_to_html_with_plugins(&file_content, &options, &plugins);

    let matter = Matter::<YAML>::new();
    let parsed = matter
        .parse::<BlogMeta>(&file_content)
        .map_err(|e| ServerFnError::new(format!("Failed to parse post: {}", e)))?;
    let meta = parsed
        .data
        .ok_or_else(|| ServerFnError::new("Post is missing frontmatter"))?;

    let document = Html::parse_fragment(&content);
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
        meta,
        content,
        headers,
        prev_post,
        next_post,
    })
}

#[server]
pub async fn get_latest_post_slug() -> Result<String, ServerFnError> {
    let posts = get_cached_posts().await?;
    posts
        .first()
        .map(|(slug, _)| slug.clone())
        .ok_or_else(|| ServerFnError::new("No posts found"))
}

#[cfg(feature = "server")]
async fn get_cached_posts() -> Result<&'static Vec<(String, BlogMeta)>, ServerFnError> {
    use gray_matter::{Matter, engine::YAML};
    use tokio::fs;

    POST_CACHE
        .get_or_try_init(|| async {
            let mut posts = Vec::new();
            let mut entries = fs::read_dir(BLOG_DIR)
                .await
                .map_err(|e| ServerFnError::new(format!("Dir error: {}", e)))?;

            let matter = Matter::<YAML>::new();

            while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    let slug = path.file_stem().unwrap().to_str().unwrap().to_string();
                    let content = fs::read_to_string(&path).await.unwrap();
                    if let Ok(Some(meta)) = matter.parse::<BlogMeta>(&content).map(|p| p.data) {
                        posts.push((slug, meta));
                    }
                }
            }
            posts.sort_by(|a, b| b.1.date.cmp(&a.1.date));

            Ok(posts)
        })
        .await
}
