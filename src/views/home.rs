use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 {
            "Hi, I'm Rui."
        }
        p {
            "I'm a software engineer with a strong bias toward systems that are coherent, minimal, and internally correct. I care about structure: in code, in architecture, and in security boundaries. If something works but feels inconsistent, I'll refactor it until it makes sense."
        }
        p {
            "I'm deeply invested in Linux and open source. I run Arch and NixOS, build my workflow around Niri, WezTerm, Fish, and Neovim, and prefer tools that are transparent and composable. I'm particularly interested in Rust for its emphasis on correctness and explicitness (including building this site as a full-stack Dioxus project)."
        }
        p {
            "My interests center on systems design and cybersecurity: understanding trust models, attack surfaces, and how small implementation details create large consequences. I'm driven by curiosity, precision, and a desire to understand how things work beneath the abstraction layer."
        }
    }
}
