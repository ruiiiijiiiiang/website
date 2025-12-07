use crate::Route;
use crate::components::{GithubIcon, ThemeToggle};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            ul {
                li { Link { to: Route::Home {}, "Home" } }
                li { Link { to: Route::Blog { id: 0 }, "Blog" } }
            }
            ul {
                li { ThemeToggle {} }
                li {
                    a {
                        href: "https://github.com/ruiiiijiiiiang",
                        target: "_blank",
                        class: "contrast",
                        aria_label: "GitHub",
                        GithubIcon {}
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
