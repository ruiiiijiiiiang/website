use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::{FaGithub, FaLinkedin};
use dioxus_free_icons::icons::fa_solid_icons::{FaFileLines, FaHouse, FaLaptopCode, FaUser};

use crate::DOMAIN;
use crate::Route;
use crate::components::{ScrollControls, ThemeToggle};

#[component]
pub fn Navbar() -> Element {
    let current_route = use_route::<Route>();
    let is_home_active = matches!(current_route, Route::Home {});
    let is_blog_active = matches!(current_route, Route::Blog {} | Route::BlogPost { .. });
    let is_projects_active = matches!(current_route, Route::Projects {});
    let is_about_active = matches!(current_route, Route::About {});
    let canonical_url = format!("{}{}", DOMAIN, current_route);
    let initial_canonical_url = use_hook(|| canonical_url.clone());

    #[cfg(target_arch = "wasm32")]
    use_effect(use_reactive((&canonical_url,), |(canonical_url,)| {
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        if let Ok(Some(link)) = document.query_selector("head > link[rel=\"canonical\"]") {
            // Update the existing head element instead of accumulating canonical links.
            let _ = link.set_attribute("href", &canonical_url);
        }
    }));

    rsx! {
        document::Link { rel: "canonical", href: initial_canonical_url }
        nav {
            ul {
                li {
                    Link {
                        to: Route::Home {},
                        class: if is_home_active { "active-nav-link" } else { "" },
                        Icon { icon: FaHouse }
                        " Home"
                    }
                }
                li {
                    Link {
                        to: Route::Blog {},
                        class: if is_blog_active { "active-nav-link" } else { "" },
                        Icon { icon: FaFileLines }
                        " Blog"
                    }
                }
                li {
                    Link {
                        to: Route::Projects {},
                        class: if is_projects_active { "active-nav-link" } else { "" },
                        Icon { icon: FaLaptopCode }
                        " Projects"
                    }
                }
                li {
                    Link {
                        to: Route::About {},
                        class: if is_about_active { "active-nav-link" } else { "" },
                        Icon { icon: FaUser }
                        " About"
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

        ScrollControls {}
        Outlet::<Route> {}
    }
}
