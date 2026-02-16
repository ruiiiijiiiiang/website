use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct BlogData {
    pub content: String,
    pub headers: Vec<HeaderLink>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct HeaderLink {
    pub id: String,
    pub title: String,
}
