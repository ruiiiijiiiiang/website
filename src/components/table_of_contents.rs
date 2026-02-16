use dioxus::prelude::*;

use crate::models::HeaderLink;

#[component]
pub fn TableOfContents(headers: Vec<HeaderLink>) -> Element {
    rsx! {
        aside { class: "table-of-contents",
            nav {
                ul {
                    for header in headers {
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
