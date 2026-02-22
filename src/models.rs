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
