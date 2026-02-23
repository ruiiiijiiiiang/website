use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::FaLinkedin;

#[component]
pub fn LinkedinLink() -> Element {
    rsx! {
        a {
            href: "https://www.linkedin.com/in/ruij/",
            target: "_blank",
            class: "outline",
            "aria-label": "link to linkedin account",
            "data-tooltip": "LinkedIn",
            "data-placement": "bottom",
            Icon {
                icon: FaLinkedin
            }
        }
    }
}
