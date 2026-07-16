use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaLinkedin};

use crate::DOMAIN;
use crate::Route;
use crate::components::ThemeToggle;

#[component]
pub fn Navbar() -> Element {
    let current_route = use_route::<Route>();
    let is_home_active = matches!(current_route, Route::Home {});
    let is_blog_active = matches!(current_route, Route::Blog {} | Route::BlogPost { .. });
    let is_projects_active = matches!(current_route, Route::Projects {});
    let canonical_url = format!("{}{}", DOMAIN, current_route);

    rsx! {
        document::Link { rel: "canonical", href: canonical_url }
        nav {
            ul {
                li {
                    Link {
                        to: Route::Home {},
                        class: if is_home_active { "active-nav-link" } else { "" },
                        "Home"
                    }
                }
                li {
                    Link {
                        to: Route::Blog {},
                        class: if is_blog_active { "active-nav-link" } else { "" },
                        "Blog"
                    }
                }
                li {
                    Link {
                        to: Route::Projects {},
                        class: if is_projects_active { "active-nav-link" } else { "" },
                        "Projects"
                    }
                }
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
