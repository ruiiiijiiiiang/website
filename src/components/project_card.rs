use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::icons::fa_solid_icons::FaFileImage;

use crate::models::{PreviewKind, PreviewSource, Project};

#[component]
pub fn ProjectCard(project: Project) -> Element {
    let lang_class = match project.language {
        "rust" => "rust",
        "nix" => "nix",
        "python" => "python",
        _ => "other",
    };
    let preview = project.preview.map(|preview| {
        let images = match preview.source {
            PreviewSource::Remote(url) => vec![(url.to_string(), "")],
            PreviewSource::ThemedLocal { light, dark } => vec![
                (light.to_string(), "project-preview-theme-light"),
                (dark.to_string(), "project-preview-theme-dark"),
            ],
        };
        let (label, class) = match preview.kind {
            PreviewKind::Terminal => ("Terminal UI preview", "project-preview project-preview-tui"),
            PreviewKind::Architecture => (
                "Architecture preview",
                "project-preview project-preview-diagram",
            ),
            PreviewKind::Workflow => (
                "Workflow preview",
                "project-preview project-preview-diagram",
            ),
        };
        (preview, images, label, class)
    });

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
                                span { class: "technology-tag", "{technology}" }
                            }
                        }
                    }
                    if let Some((preview, preview_images, preview_label, preview_class)) = &preview {
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
                            for (preview_url, theme_class) in preview_images {
                                figure { class: "{preview_class} {theme_class}",
                                    img {
                                        src: "{preview_url}",
                                        alt: "{preview.alt}",
                                        loading: "lazy",
                                    }
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
