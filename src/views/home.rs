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
            class: "terminal-cursor",
            "Hi, I'm Rui."
        }
        h5 {
            "Open-Source Enthusiast. Linux fan. Rustacean. AUR and nixpkgs maintainer."
        }
        hr { }
        p {
            "I'm a software engineer with a strong bias toward systems that are coherent, minimal, and internally correct. I care about structure: in code, in architecture, and in security boundaries."
        }
        p {
            "My interests include systems design, networking, and cybersecurity. I'm driven by curiosity, precision, and a desire to understand how things work beneath the abstraction layer."
        }
        p {
            "My workflow is built around Niri, WezTerm, Fish, and Neovim. I use Arch, and NixOS, btw."
        }
        FastfetchCard {}
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
