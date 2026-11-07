use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaEnvelope;

use crate::components::FastfetchCard;

const HOME_CSS: Asset = asset!("../../assets/home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        document::Title { "Rui's Personal Website" }

        h1 {
            "Hi, I'm Rui."
        }
        h5 {
            "Software engineer focused on platform engineering, cloud automation, and secure Linux systems."
        }

        hr { }

        p {
            "I’m a full-stack software engineer with over seven years of professional experience across multiple startups. My main stack consists of "
            span { class: "technology-tag", "TypeScript" }
            ", "
            span { class: "technology-tag", "React" }
            ", and "
            span { class: "technology-tag", "Node.js" }
            "."
        }
        p {
            "My open-source work focuses on reproducible infrastructure, monitoring, and secure operations. I build Linux automation and developer tools with "
            span { class: "technology-tag", "Nix" }
            ", "
            span { class: "technology-tag", "Rust" }
            ", and "
            span { class: "technology-tag", "Python" }
            "."
        }
        p {
            "Based in Houston, Texas. Let’s talk about software engineering, DevOps, and SRE opportunities."
        }

        hr { }

        FastfetchCard {}

        p { class: "live-system-notice",
            "# Live system telemetry from the NixOS server hosting this site; refreshes on page load."
        }

        hr { }

        footer {
            a {
                href: "mailto:me@ruijiang.me",
                target: "_blank",
                "aria-label": "link to email",
                "data-tooltip": "Email",
                "data-placement": "bottom",
                Icon {
                    icon: FaEnvelope
                }
                " me@ruijiang.me"
            }
        }
    }
}
