use dioxus::prelude::*;

use crate::Route;
use crate::backend::get_blog_posts;

const BLOG_CSS: Asset = asset!("../../assets/blog.css");

#[component]
pub fn Blog() -> Element {
    let posts = use_loader(get_blog_posts)?();

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
        document::Title { "Rui's Blog Posts" }

        h1 { "Blog Posts" }

        ul {
            class: "blog-list",
            for post in posts {
                li {
                    Link {
                        class: "blog-item-title",
                        to: Route::BlogPost { slug: post.slug },
                        "{post.title}"
                    }
                    span { class: "blog-item-date", "{post.date.format(\"%m/%d/%Y\")}" }
                }
            }
        }
    }
}
