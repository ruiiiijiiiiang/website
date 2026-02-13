use dioxus::prelude::*;
use scraper::{Html, Selector};

#[derive(PartialEq, Clone, Debug)]
struct HeaderLink {
    id: String,
    title: String,
}

#[component]
pub fn TableOfContents(content: String) -> Element {
    let headers = use_memo(use_reactive(&content, |content| extract_headers(&content)));

    rsx! {
        aside { class: "table-of-contents",
            nav {
                ul {
                    for header in headers.read().iter() {
                        li {
                            Link {
                                to: "#{header.id}",
                                "{header.title}"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn extract_headers(html_content: &str) -> Vec<HeaderLink> {
    let document = Html::parse_fragment(html_content);
    let selector = Selector::parse("h3[id]").unwrap();

    document
        .select(&selector)
        .map(|element| {
            let id = element.value().attr("id").unwrap_or_default().to_string();
            let title = element.text().collect::<Vec<_>>().join("");
            HeaderLink { id, title }
        })
        .collect()
}
