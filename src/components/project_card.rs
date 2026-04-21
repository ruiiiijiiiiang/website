use dioxus::prelude::*;

use crate::models::Project;

#[component]
pub fn ProjectCard(project: Project) -> Element {
    let color = match project.language {
        "rust" => "#dea584",
        "nix" => "#7e7eff",
        _ => "#cccccc",
    };

    rsx! {
        article {
            header {
                nav {
                    ul {
                        li { "{project.name}" }
                    }
                    ul {
                        li { span { class: "repo-language-color", style: "background-color: {color};" } }
                        li { "{project.language}" }
                    }
                }
            }
            "{project.description}"
            footer {
                a {
                    href: project.link,
                    target: "_blank",
                    "aria-label": "link to {project.name} github repository",
                    "View on GitHub"
                }
            }
        }
    }
}
