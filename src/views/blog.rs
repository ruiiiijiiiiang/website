use dioxus::prelude::*;

use crate::backend::{get_blog_content, get_blog_count};
use crate::components::{Footer, TableOfContents};

const BLOG_CSS: Asset = asset!("../../assets/blog.css");

#[component]
pub fn Blog(id: ReadSignal<usize>) -> Element {
    let blog_count = use_loader(get_blog_count)?;

    let blog_content = use_loader(move || get_blog_content(id()))?;

    use_effect(move || {
        let _ = document::eval(
            r#"
                requestAnimationFrame(() => {
                    if (window.Prism) {
                        window.Prism.highlightAll();
                    }
                });
            "#,
        );
    });

    rsx! {
        link { rel: "stylesheet", href: BLOG_CSS }
        div {
            class: "blog",
            TableOfContents {
                content: blog_content
            }
            div {
                class: "blog-content",
                dangerous_inner_html: "{blog_content}"
            }
            BackToTop {}
        }
        Footer {
            current: id(),
            count: blog_count()
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
            "↑ Top"
        }
    }
}
