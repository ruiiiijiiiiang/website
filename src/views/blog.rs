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
            Ok(content) => parse_basic_markdown(&content),
            Err(_) => String::from("Error loading content."),
        }
    });

    use_effect(move || {
        let _ = &blog_content;
        let _ = document::eval(
            r#"
            setTimeout(() => {
                if (window.Prism) {
                    window.Prism.highlightAll();
                }
            }, 50);
        "#,
        );
    });

    rsx! {
        match &*blog_content.read_unchecked() {
            Some(html_string) => rsx! {
                div {
                    id: "blog",
                    dangerous_inner_html: "{html_string}"
                }
                hr {}
                Footer {}
            },
            None => rsx! {
                div {
                    "aria-busy": true,
                    "aria-label": "Please wait..."
                }
            }
        }
    }
}
