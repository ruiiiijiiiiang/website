use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaArrowLeft, FaArrowRight};

use crate::Route;
use crate::models::BlogLink;

#[component]
pub fn Footer(prev_post: Option<BlogLink>, next_post: Option<BlogLink>) -> Element {
    rsx! {
        footer {
            nav {
                ul {
                    li {
                        if let Some(prev) = prev_post {
                            Link {
                                to: Route::BlogPost { slug: prev.slug },
                                "aria-label": "previous post",
                                Icon {
                                    icon: FaArrowLeft
                                }
                                " Previous"
                            }
                        }
                    }
                }
                ul {
                    li {
                        if let Some(next) = next_post {
                            Link {
                                to: Route::BlogPost { slug: next.slug },
                                "aria-label": "next post",
                                "Next "
                                Icon {
                                    icon: FaArrowRight
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
