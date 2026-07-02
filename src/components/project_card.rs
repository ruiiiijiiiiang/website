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
                    class: "project-card-header",
                    ul {
                        li { "[ {project.name} ]" }
                    }
                    ul {
                        li { span { class: "repo-language-color", style: "background-color: {color};" } }
                        li { "{project.language}" }
                    }
                }
            }
            "{project.description}"
            if let Some(screenshot_url) = project.screenshot {
                details {
                    style: "margin-top: 1rem; border: none; padding: 0;",
                    summary {
                        style: "cursor: pointer; font-size: 0.9rem; color: var(--pico-secondary);",
                        "View Screenshot"
                    }
                    img {
                        src: screenshot_url,
                        alt: "Screenshot of {project.name}",
                        style: "max-width: 100%; margin-top: 0.5rem; border-radius: 4px; display: block;"
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
