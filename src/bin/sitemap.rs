use chrono::NaiveDate;
use gray_matter::{Matter, engine::YAML};
use serde::{self, Deserialize, Serialize};
use std::fs;

mod date {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%m/%d/%Y";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct BlogMeta {
    pub title: String,
    #[serde(with = "date")]
    pub date: NaiveDate,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

const BLOG_DIR: &str = "./blog";
const DOMAIN: &str = "https://public.ruijiang.me";
const PUBLIC_DIR: &str = "target/dx/website/release/web/public";

fn main() {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
    );

    xml.push_str(&format!(
        "  <url>\n    <loc>{}</loc>\n    <changefreq>weekly</changefreq>\n  </url>\n",
        DOMAIN
    ));

    let matter = Matter::<YAML>::new();
    let paths = fs::read_dir(BLOG_DIR).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let slug = path.file_stem().unwrap().to_str().unwrap();
            let content = fs::read_to_string(&path).unwrap();

            let parsed = matter.parse::<BlogMeta>(&content).unwrap();
            let meta = parsed.data.unwrap();
            let date = meta.date.format("%Y-%m-%d");
            xml.push_str(&format!(
                "  <url>\n    <loc>{}/blog/{}</loc>\n    <lastmod>{}</lastmod>\n  </url>\n",
                DOMAIN, slug, date
            ));
        }
    }

    xml.push_str("</urlset>");

    let sitemap_path = format!("{}/sitemap.xml", PUBLIC_DIR);
    fs::write(&sitemap_path, xml).unwrap();
    println!("✅ Successfully generated {}", sitemap_path);

    let robots_content = format!("User-agent: *\nAllow: /\nSitemap: {}/sitemap.xml\n", DOMAIN);
    let robots_path = format!("{}/robots.txt", PUBLIC_DIR);
    fs::write(&robots_path, robots_content).unwrap();
    println!("✅ Successfully generated {}", robots_path);
}
