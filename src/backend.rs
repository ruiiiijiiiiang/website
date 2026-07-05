use dioxus::prelude::*;

use crate::models::{BlogData, BlogLink, BlogMeta, FastfetchData, HeaderLink};

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
            date: n_meta.date,
        })
    } else {
        None
    };

    let prev_post = if current_index < sorted_posts.len() - 1 {
        let (p_slug, p_meta) = &sorted_posts[current_index + 1];
        Some(BlogLink {
            slug: p_slug.clone(),
            title: p_meta.title.clone(),
            date: p_meta.date,
        })
    } else {
        None
    };

    let file_path = format!("{}/{}.md", BLOG_DIR, slug);
    let file_content = fs::read_to_string(&file_path)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to read post: {}", e)))?;

    let adapter = CustomHighlighter::new("base16-ocean.dark");

    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.header_id_prefix = Some("".to_string());
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
pub async fn get_blog_posts() -> Result<Vec<BlogLink>, ServerFnError> {
    let posts = get_cached_posts().await?;
    Ok(posts
        .iter()
        .rev()
        .map(|(slug, meta)| BlogLink {
            slug: slug.clone(),
            title: meta.title.clone(),
            date: meta.date,
        })
        .collect())
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
            posts.sort_by_key(|a| std::cmp::Reverse(a.1.date));

            Ok(posts)
        })
        .await
}

#[server]
pub async fn get_fastfetch_data() -> Result<FastfetchData, ServerFnError> {
    use std::fs;
    use std::process::Command;

    let cpu_model = fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|info| {
            info.lines()
                .find(|line| line.starts_with("model name"))
                .and_then(|line| line.split(':').nth(1))
                .map(|name| name.trim().to_string())
        })
        .unwrap_or_else(|| "Unknown CPU".to_string());

    let cores = fs::read_to_string("/proc/cpuinfo")
        .map(|info| {
            info.lines()
                .filter(|line| line.starts_with("processor"))
                .count()
                .max(1)
        })
        .unwrap_or(1);

    let load_1m = fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| {
            s.split_whitespace()
                .next()
                .and_then(|val| val.parse::<f64>().ok())
        })
        .unwrap_or(0.0);

    let cpu_load = (((load_1m / cores as f64) * 100.0).round() as u8).min(100);

    let uptime_secs = fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| {
            s.split_whitespace()
                .next()
                .and_then(|val| val.parse::<f64>().ok())
        })
        .unwrap_or(0.0) as u64;
    let days = uptime_secs / 86400;
    let hours = (uptime_secs % 86400) / 3600;
    let minutes = (uptime_secs % 3600) / 60;
    let uptime = if days > 0 {
        format!("{}d, {}h, {}m", days, hours, minutes)
    } else {
        format!("{}h, {}m", hours, minutes)
    };

    let mut mem_total = 1;
    let mut mem_available = 0;
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                mem_total = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(1);
            } else if line.starts_with("MemAvailable:") {
                mem_available = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(0);
            }
        }
    }
    let ram_pct = (((mem_total - mem_available) as f64 / mem_total as f64) * 100.0) as u8;

    let disk_pct = Command::new("df")
        .args(&["-h", "/"])
        .output()
        .ok()
        .and_then(|output| {
            String::from_utf8(output.stdout).ok().and_then(|stdout| {
                stdout.lines().nth(1).and_then(|line| {
                    line.split_whitespace()
                        .nth(4)
                        .and_then(|pct| pct.trim_end_matches('%').parse::<u8>().ok())
                })
            })
        })
        .unwrap_or(0);

    let os_name = fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|content| {
            content
                .lines()
                .find(|line| line.starts_with("PRETTY_NAME="))
                .and_then(|line| line.split('=').nth(1))
                .map(|name| name.trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "NixOS".to_string());

    let kernel_release = fs::read_to_string("/proc/sys/kernel/osrelease")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());
    let kernel = format!("Linux {}", kernel_release);

    let os_age_days = fs::metadata("/etc/NIXOS")
        .or_else(|_| fs::metadata("/etc/hostname"))
        .or_else(|_| fs::metadata("/lost+found"))
        .and_then(|m| m.modified().or_else(|_| m.created()))
        .ok()
        .map(|time| {
            std::time::SystemTime::now()
                .duration_since(time)
                .map(|d| d.as_secs() / 86400)
                .unwrap_or(120)
        })
        .unwrap_or(120);
    let os_age = format!("{} days", os_age_days);

    // 9. Packages count (NixOS fallback to /usr/bin)
    let sys_packages = fs::read_dir("/run/current-system/sw/bin")
        .or_else(|_| fs::read_dir("/usr/bin"))
        .map(|entries| entries.count())
        .ok();

    let packages = match sys_packages {
        Some(count) if count > 0 => count.to_string(),
        _ => "Unknown".to_string(),
    };

    let fetched_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(FastfetchData {
        cpu_model,
        cpu_load,
        ram_pct,
        disk_pct,
        uptime,
        os_name,
        kernel,
        os_age,
        packages,
        fetched_at,
    })
}
