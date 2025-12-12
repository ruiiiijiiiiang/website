use dioxus::prelude::*;
use pulldown_cmark::{Parser, html};

use crate::backend::get_blog;
use crate::components::Footer;

pub fn parse_basic_markdown(markdown_input: &str) -> String {
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[component]
pub fn Blog(id: i32) -> Element {
    let blog_content = use_resource(move || async move {
        let blog_res = get_blog(id).await;
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
        Footer {}
    }
}
