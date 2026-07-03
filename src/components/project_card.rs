use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaFileImage;

use crate::models::Project;

#[component]
pub fn ProjectCard(project: Project) -> Element {
    let lang_class = match project.language {
        "rust" => "rust",
        "nix" => "nix",
        _ => "other",
    };

    rsx! {
        div {
            class: "project-card-container",
            article {
                class: "project-card",
                div {
                    class: "cli-output",
                    p {
                        span { class: "cli-label", "[NAME]       " }
                        span { class: "cli-value", "{project.name}" }
                    }
                    p {
                        span { class: "cli-label", "[LANGUAGE]   " }
                        span { class: "cli-value repo-language",
                            span { class: "repo-language-color {lang_class}" }
                            "{project.language}"
                        }
                    }
                    p {
                        span { class: "cli-label", "[SYNOPSIS]   " }
                        span { class: "cli-value", "{project.description}" }
                    }
                    if let Some(screenshot_url) = project.screenshot {
                        details {
                            class: "cli-details",
                            summary {
                                Icon {
                                    icon: FaFileImage,
                                    width: 16,
                                    height: 16,
                                    class: "folder-icon",
                                }
                                " Attachment: {project.name}-preview.png (click to render)"
                            }
                            img {
                                src: screenshot_url,
                                alt: "Screenshot of {project.name}",
                            }
                        }
                    }
                }
                footer {
                    class: "cli-footer",
                    span { class: "prompt-symbol", "$ " }
                    a {
                        href: project.link,
                        target: "_blank",
                        class: "git-clone-link",
                        "aria-label": "link to {project.name} github repository",
                        "open {project.link}"
                    }
                }
            }
        }
    }
}
