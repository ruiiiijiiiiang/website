use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;

#[component]
pub fn GithubLink() -> Element {
    rsx! {
        a {
            href: "https://github.com/ruiiiijiiiiang",
            target: "_blank",
            button {
                class: "outline",
                style: "border: none",
                "data-tooltip": "Github",
                "data-placement": "bottom",
                Icon {
                    icon: FaGithub
                }
            }
        }
    }
}
