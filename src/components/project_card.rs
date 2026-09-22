use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::icons::fa_solid_icons::FaFileImage;

use crate::models::Project;

#[component]
pub fn ProjectCard(project: Project) -> Element {
    let lang_class = match project.language {
        "rust" => "rust",
        "nix" => "nix",
        _ => "other",
    };
    let has_tui_preview = project.screenshot.is_some();
    let screenshot_alt = project.screenshot_alt.unwrap_or("Project preview");
    let preview_label = if project.category == "Platform Engineering" {
        "Architecture preview"
    } else {
        "Terminal UI preview"
    };

    rsx! {
        div {
            class: "project-card-container",
            article {
                class: "project-card",
                header {
                    class: "project-card-header",
                    div { class: "project-card-context",
                        span { class: "project-category", "{project.category}" }
                    }
                    span { class: "project-language",
                        span { class: "repo-language-color {lang_class}" }
                        "{project.language}"
                    }
                }
                h2 { class: "project-name", "{project.name}" }
                p { class: "project-tagline", "{project.tagline}" }
                div {
                    class: "project-card-body",
                    div {
                        class: "cli-output",
                        span { class: "cli-label", "[PROBLEM]" }
                        span { class: "cli-value", "{project.problem}" }
                        span { class: "cli-label", "[BUILT]" }
                        span { class: "cli-value", "{project.solution}" }
                        span { class: "cli-label", "[HIGHLIGHTS]" }
                        div { class: "cli-value project-signals",
                            ul {
                                for highlight in project.highlights {
                                    li { "{highlight}" }
                                }
                            }
                        }
                        span { class: "cli-label", "[KEYWORDS]" }
                        div { class: "cli-value project-stack-items",
                            for technology in project.stack {
                                span { class: "project-stack-item", "{technology}" }
                            }
                        }
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
                                "{preview_label}"
                            }
                            figure { class: if has_tui_preview { "project-preview project-preview-tui" } else { "project-preview" },
                                img {
                                    src: screenshot_url,
                                    alt: "{screenshot_alt}",
                                    loading: "lazy",
                                }
                            }
                        }
                    }
                }
                footer {
                    class: "cli-footer",
                    span { class: "prompt-symbol", "❯ " }
                    a {
                        href: project.link,
                        target: "_blank",
                        class: "git-clone-link",
                        rel: "noreferrer",
                        "aria-label": "Open the {project.name} source repository",
                        Icon {
                            icon: FaGithub
                        }
                        " view source ↗"
                    }
                }
            }
        }
    }
}
