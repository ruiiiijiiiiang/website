use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaArrowLeft, FaArrowRight, FaArrowUp};

use crate::Route;
use crate::backend::get_blog_data;

const BLOG_CSS: Asset = asset!("../../assets/blog.css");

#[cfg(target_arch = "wasm32")]
#[allow(dead_code)]
struct ScrollListener {
    window: web_sys::Window,
    closure: Option<wasm_bindgen::prelude::Closure<dyn FnMut(web_sys::Event)>>,
}

#[cfg(not(target_arch = "wasm32"))]
struct ScrollListener;

#[cfg(target_arch = "wasm32")]
impl Drop for ScrollListener {
    fn drop(&mut self) {
        if let Some(closure) = self.closure.take() {
            use wasm_bindgen::JsCast;
            let _ = self
                .window
                .remove_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
        }
    }
}

#[component]
pub fn BlogPost(slug: ReadSignal<String>) -> Element {
    let blog_data = use_loader(move || get_blog_data(slug()))?();
    let mut scroll_percentage = use_signal(|| 0.0);
    let mut active_header = use_signal(String::new);
    let mut _listener = use_signal(|| None::<ScrollListener>);

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use wasm_bindgen::prelude::*;
            use web_sys::Element;

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let on_scroll = Closure::wrap(Box::new(move |_: web_sys::Event| {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                let doc_element = document.document_element().unwrap();
                let scroll_top = doc_element.scroll_top();
                let scroll_height = doc_element.scroll_height();
                let client_height = doc_element.client_height();

                let max_scroll = scroll_height - client_height;
                let percent = if max_scroll > 0 {
                    (scroll_top as f64 / max_scroll as f64) * 100.0
                } else {
                    0.0
                };
                scroll_percentage.set(percent);

                let headers = document
                    .query_selector_all(".blog-content h2, .blog-content h3")
                    .unwrap();
                let mut active_id = String::new();

                let max_slide_range = std::cmp::min(max_scroll, client_height);
                let remaining_slide = max_scroll - scroll_top;
                let slide_progress = if max_slide_range > 0 {
                    1.0 - (remaining_slide as f64 / max_slide_range as f64).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let threshold = 120.0 + (client_height as f64 - 120.0) * slide_progress;

                for i in 0..headers.length() {
                    if let Some(node) = headers.get(i) {
                        if let Ok(el) = node.dyn_into::<Element>() {
                            let rect = el.get_bounding_client_rect();
                            if rect.top() <= threshold {
                                if let Ok(Some(anchor)) = el.query_selector("a[id]") {
                                    if let Some(id) = anchor.get_attribute("id") {
                                        active_id = id;
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                active_header.set(active_id);
            }) as Box<dyn FnMut(web_sys::Event)>);

            window
                .add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref())
                .unwrap();

            // Trigger initial scroll calculation
            let doc_element = document.document_element().unwrap();
            let scroll_top = doc_element.scroll_top();
            let scroll_height = doc_element.scroll_height();
            let client_height = doc_element.client_height();
            let max_scroll = scroll_height - client_height;
            let percent = if max_scroll > 0 {
                (scroll_top as f64 / max_scroll as f64) * 100.0
            } else {
                0.0
            };
            scroll_percentage.set(percent);

            let max_slide_range = std::cmp::min(max_scroll, client_height);
            let remaining_slide = max_scroll - scroll_top;
            let slide_progress = if max_slide_range > 0 {
                1.0 - (remaining_slide as f64 / max_slide_range as f64).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let threshold = 120.0 + (client_height as f64 - 120.0) * slide_progress;

            let headers = document
                .query_selector_all(".blog-content h2, .blog-content h3")
                .unwrap();
            let mut active_id = String::new();
            for i in 0..headers.length() {
                if let Some(node) = headers.get(i) {
                    if let Ok(el) = node.dyn_into::<Element>() {
                        let rect = el.get_bounding_client_rect();
                        if rect.top() <= threshold {
                            if let Ok(Some(anchor)) = el.query_selector("a[id]") {
                                if let Some(id) = anchor.get_attribute("id") {
                                    active_id = id;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            active_header.set(active_id);

            _listener.set(Some(ScrollListener {
                window,
                closure: Some(on_scroll),
            }));
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
        document::Title { "Rui's Blog: {blog_data.meta.title}" }

        div {
            class: "scroll-progress-bar",
            style: "width: {scroll_percentage}%;",
        }

        div {
            class: "blog",
            aside { class: "table-of-contents",
                nav {
                    ul {
                        for header in {blog_data.headers} {
                            li {
                                Link {
                                    to: "#{header.id}",
                                    class: if active_header() == header.id { "active-toc-link" } else { "" },
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
