use dioxus::prelude::*;

use crate::Route;
use crate::backend::get_blog_posts;

#[component]
pub fn Blog() -> Element {
    let posts = use_loader(get_blog_posts)?();

    rsx! {
        document::Title { "Rui's Blog Posts" }

        h2 {
            "Blog Posts"
        }
        ul {
            for post in posts {
                li {
                    Link {
                        to: Route::BlogPost { slug: post.slug },
                        "{post.title}"
                    }
                    span { " - {post.date.format(\"%B %d, %Y\").to_string()}" }
                }
            }
        }
    }
}
