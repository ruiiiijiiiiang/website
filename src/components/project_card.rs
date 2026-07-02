use dioxus::prelude::*;

use crate::models::Project;

#[component]
pub fn ProjectCard(project: Project) -> Element {
    let lang_class = match project.language {
        "rust" => "rust",
        "nix" => "nix",
        _ => "other",
    };

    rsx! {
        article {
            header {
                nav {
                    class: "project-card-header",
                    ul {
                        li { "[ {project.name} ]" }
                    }
                    ul {
                        li { span { class: "repo-language-color {lang_class}" } }
                        li { "{project.language}" }
                    }
                }
            }
            "{project.description}"
            if let Some(screenshot_url) = project.screenshot {
                details {
                    summary {
                        "View Screenshot"
                    }
                    img {
                        src: screenshot_url,
                        alt: "Screenshot of {project.name}",
                    }
                }
            }
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
