use dioxus::prelude::*;

use crate::backend::{get_blog_content, get_blog_count};
use crate::components::Footer;

#[component]
pub fn Blog(id: ReadSignal<usize>) -> Element {
    let blog_count = use_loader(get_blog_count)?;

    let blog_content = use_loader(move || get_blog_content(id()))?;

    use_effect(move || {
        let _ = document::eval(
            r#"
                requestAnimationFrame(() => {
                    if (window.Prism) {
                        window.Prism.highlightAll();
                    }
                });
            "#,
        );
    });

    rsx! {
        div {
            id: "blog",
            dangerous_inner_html: "{blog_content}"
        }
        Footer {
            current: id(),
            count: blog_count()
        }
    }
}
