use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaLinkedin};

use crate::Route;
use crate::components::ThemeToggle;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            ul {
                li { Link { to: Route::Home {}, "Home" } }
                li { Link { to: Route::Blog {}, "Blog" } }
                li { Link { to: Route::Projects {}, "Projects" } }
            }
            ul {
                li {
                    a {
                        href: "https://www.linkedin.com/in/ruij/",
                        target: "_blank",
                        class: "outline",
                        "aria-label": "link to linkedin account",
                        "data-tooltip": "LinkedIn",
                        "data-placement": "bottom",
                        Icon {
                            icon: FaLinkedin
                        }
                    }
                }
                li {
                    a {
                        href: "https://github.com/ruiiiijiiiiang",
                        target: "_blank",
                        class: "outline",
                        "aria-label": "link to github account",
                        "data-tooltip": "Github",
                        "data-placement": "bottom",
                        Icon {
                            icon: FaGithub
                        }
                    }
                }
                li { ThemeToggle {} }
            }
        }

        Outlet::<Route> {}
    }
}
