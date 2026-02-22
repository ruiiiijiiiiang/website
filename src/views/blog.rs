use dioxus::prelude::*;

use crate::backend::get_blog_data;
use crate::components::{BackToTop, Footer, TableOfContents};

const BLOG_CSS: Asset = asset!("../../assets/blog.css");

#[component]
pub fn Blog(slug: ReadSignal<String>) -> Element {
    let blog_data = use_loader(move || get_blog_data(slug()))?();

    rsx! {
        link { rel: "stylesheet", href: BLOG_CSS }
        div {
            class: "blog",
            TableOfContents {
                headers: blog_data.headers
            }
            div {
                class: "blog-content",
                h1 {
                    "{blog_data.meta.title}"
                }
                h4 {
                    "{blog_data.meta.date.format(\"%B %d, %Y\").to_string()}"
                }
                hr { }
                div {
                    dangerous_inner_html: "{blog_data.content}"
                }
            }
            BackToTop {}
        }
        Footer {
            prev_post: blog_data.prev_post,
            next_post: blog_data.next_post,
        }
    }
}
