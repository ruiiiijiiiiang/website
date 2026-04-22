use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaEnvelope;

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Rui's Personal Website" }

        h1 {
            "Hi, I'm Rui."
        }
        h5 {
            "Open-Source Enthusiast. Linux lover. Rustacean. AUR and nixpkgs maintainer."
        }
        hr { }
        p {
            "I'm a software engineer with a strong bias toward systems that are coherent, minimal, and internally correct. I care about structure: in code, in architecture, and in security boundaries."
        }
        p {
            "My interests center around systems design and cybersecurity: understanding trust models, attack surfaces, and how small implementation details create large consequences. I'm driven by curiosity, precision, and a desire to understand how things work beneath the abstraction layer."
        }
        p {
            "I build my workflow around Niri, WezTerm, Fish, and Neovim. I'm particularly interested in Rust for its emphasis on correctness and explicitness (which led me to building this site as a full-stack Dioxus project)."
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
