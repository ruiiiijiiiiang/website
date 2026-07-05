use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::utils::date;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BlogData {
    pub meta: BlogMeta,
    pub content: String,
    pub headers: Vec<HeaderLink>,
    pub prev_post: Option<BlogLink>,
    pub next_post: Option<BlogLink>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BlogLink {
    pub slug: String,
    pub title: String,
    #[serde(with = "date")]
    pub date: NaiveDate,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BlogMeta {
    pub title: String,
    #[serde(with = "date")]
    pub date: NaiveDate,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HeaderLink {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub name: &'static str,
    pub language: &'static str,
    pub description: &'static str,
    pub link: &'static str,
    pub screenshot: Option<&'static str>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct FastfetchData {
    pub cpu_model: String,
    pub cpu_load: u8,
    pub ram_pct: u8,
    pub disk_pct: u8,
    pub uptime: String,
    pub os_name: String,
    pub kernel: String,
    pub os_age: String,
    pub packages: String,
    pub fetched_at: String,
}
