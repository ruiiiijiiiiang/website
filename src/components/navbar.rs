use dioxus::prelude::*;

use crate::Route;
use crate::backend::get_latest_post_slug;
use crate::components::{GithubLink, LinkedinLink, ThemeToggle};

#[component]
pub fn Navbar() -> Element {
    let latest_slug = use_resource(get_latest_post_slug);
    rsx! {
        nav {
            ul {
                li { Link { to: Route::Home {}, "Home" } }
                match latest_slug.read().as_ref() {
                    Some(Ok(slug)) => rsx! {
                        li {
                            Link {
                                to: Route::Blog { slug: slug.clone() },
                                "Blog"
                            }
                        }
                    },
                    Some(Err(_)) => rsx! {
                        li { span { "Blog" } }
                    },
                    None => rsx! {
                        li { span { "Loading..." } }
                    }
                }
            }
            ul {
                li { LinkedinLink {} }
                li { GithubLink {} }
                li { ThemeToggle {} }
            }
        }

        Outlet::<Route> {}
    }
}
