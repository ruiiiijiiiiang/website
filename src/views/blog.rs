use dioxus::prelude::*;
use pulldown_cmark::{Parser, html};

use crate::backend::{get_blog, get_blog_count};
use crate::components::Footer;

pub fn parse_basic_markdown(markdown_input: &str) -> String {
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[component]
pub fn Blog(id: ReadSignal<usize>) -> Element {
    let blog_count = use_resource(move || async move { get_blog_count().await });

    let blog_content = use_resource(move || async move {
        let current_id = id();
        let blog_res = get_blog(current_id).await;
        match blog_res {
            Ok(content) => Ok(parse_basic_markdown(&content)),
            Err(_) => Err("Error loading content."),
        }
    });

    use_effect(move || {
        let content_state = blog_content.read();
        if let Some(Ok(_)) = &*content_state {
            let _ = document::eval(
                r#"
                    requestAnimationFrame(() => {
                        if (window.Prism) {
                            window.Prism.highlightAll();
                        }
                    });
                "#,
            );
        }
    });

    rsx! {
        match &*blog_content.read_unchecked() {
            Some(Ok(html_string)) => rsx! {
                div {
                    id: "blog",
                    dangerous_inner_html: "{html_string}"
                }
                hr {}
            },
            Some(Err(err)) => rsx! {
                div {
                    "{err}"
                }
            },
            None => rsx! {
                div {
                    "aria-busy": true,
                    "aria-label": "Please wait..."
                }
            }
        }
        Footer {
            current: id(),
            count: match &*blog_count.read_unchecked() {
                Some(Ok(count)) => *count,
                _ => 0,
            }
        }
    }
}
