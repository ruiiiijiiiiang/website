use dioxus::prelude::*;

use crate::backend::{get_blog_count, get_blog_data};
use crate::components::{Footer, TableOfContents};

const BLOG_CSS: Asset = asset!("../../assets/blog.css");

#[component]
pub fn Blog(id: ReadSignal<usize>) -> Element {
    let blog_count = use_loader(get_blog_count)?();

    let blog_data = use_loader(move || get_blog_data(id()))?();

    rsx! {
        link { rel: "stylesheet", href: BLOG_CSS }
        div {
            class: "blog",
            TableOfContents {
                headers: blog_data.headers
            }
            div {
                class: "blog-content",
                dangerous_inner_html: "{blog_data.content}"
            }
            BackToTop {}
        }
        Footer {
            current: id(),
            count: blog_count
        }
    }
}

#[component]
pub fn BackToTop() -> Element {
    rsx! {
        button {
            class: "back-to-top",
            onclick: move |_| {
                spawn(async move {
                    let _ = document::eval("window.scrollTo({ top: 0 })").await;
                });
            },
            "Top"
        }
    }
}
