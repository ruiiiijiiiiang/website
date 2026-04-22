use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaArrowLeft, FaArrowRight, FaArrowUp};

use crate::Route;
use crate::backend::get_blog_data;

#[component]
pub fn BlogPost(slug: ReadSignal<String>) -> Element {
    let blog_data = use_loader(move || get_blog_data(slug()))?();

    rsx! {
        document::Title { "Rui's Blog: {blog_data.meta.title}" }

        div {
            class: "blog",
            aside { class: "table-of-contents",
                nav {
                    ul {
                        for header in {blog_data.headers} {
                            li {
                                Link {
                                    to: "#{header.id}",
                                    "aria-label": "{header.title}",
                                    "{header.title}"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "blog-content",
                h1 {
                    "{blog_data.meta.title}"
                }
                h4 {
                    "{blog_data.meta.date.format(\"%B %d, %Y\").to_string()}"
                }
                hr { }
                div {
                    dangerous_inner_html: "{blog_data.content}"
                }
            }
            button {
                class: "back-to-top",
                "aria-label": "back to top",
                onclick: move |_| {
                    spawn(async move {
                        let _ = document::eval("window.scrollTo({ top: 0 })").await;
                    });
                },
                Icon {
                    icon: FaArrowUp
                }
            }
        }
        hr { }
        footer {
            nav {
                ul {
                    li {
                        if let Some(prev) = blog_data.prev_post {
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
                        if let Some(next) = blog_data.next_post {
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
