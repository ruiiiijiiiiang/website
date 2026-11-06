use dioxus::prelude::*;

#[component]
pub fn ProjectMeta(category: String, language: String) -> Element {
    let lang_class = match language.as_str() {
        "rust" => "rust",
        "nix" => "nix",
        "python" => "python",
        _ => "other",
    };

    rsx! {
        div { class: "project-meta",
            span { class: "project-category", "{category}" }
            span { class: "project-language",
                span { class: "repo-language-color {lang_class}" }
                "{language}"
            }
        }
    }
}
