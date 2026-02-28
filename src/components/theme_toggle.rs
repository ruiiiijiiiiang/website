use dioxus::prelude::*;
use strum_macros::Display;

const THEME_TOGGLE_CSS: Asset = asset!("../../assets/theme_toggle.css");

#[derive(Clone, Copy, PartialEq, Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme = use_signal(|| Theme::Dark);
    rsx! {
        link { rel: "stylesheet", href: THEME_TOGGLE_CSS }
        button {
            class: "outline",
            "aria-label": "toggle theme",
            "data-tooltip": "toggle theme",
            "data-placement": "bottom",
            onclick: move |_| {
                let next_theme = theme().toggle();
                theme.set(next_theme);
                #[cfg(target_arch = "wasm32")]
                {
                    if let Some(window) = web_sys::window()
                    && let Some(document) = window.document()
                    && let Some(html) = document.document_element() {
                        let _ = html.set_attribute("data-theme", &next_theme.to_string());
                    }
                }
            },
            ThemeIcon { moon: theme() == Theme::Dark }
        }
    }
}

#[component]
pub fn ThemeIcon(moon: bool) -> Element {
    rsx! {
        svg {
            view_box: "0 0 32 32",
            width: "24",
            height: "24",
            fill: "currentColor",
            "class": format!("icon-theme-toggle {}", if moon { "moon" } else { "" }),
            clipPath {
                id: "theme-toggle-cutout",
                path {
                    d: "M0-11h25a1 1 0 0017 13v30H0Z"
                }
            }
            g {
                clip_path: "url(#theme-toggle-cutout)",
                circle {
                    cx: "16",
                    cy: "16",
                    r: "8.4"
                }
                path {
                    d: "M18.3 3.2c0 1.3-1 2.3-2.3 2.3s-2.3-1-2.3-2.3S14.7.9 16 .9s2.3 1 2.3 2.3zm-4.6 25.6c0-1.3 1-2.3 2.3-2.3s2.3 1 2.3 2.3-1 2.3-2.3 2.3-2.3-1-2.3-2.3zm15.1-10.5c-1.3 0-2.3-1-2.3-2.3s1-2.3 2.3-2.3 2.3 1 2.3 2.3-1 2.3-2.3 2.3zM3.2 13.7c1.3 0 2.3 1 2.3 2.3s-1 2.3-2.3 2.3S.9 17.3.9 16s1-2.3 2.3-2.3zm5.8-7C9 7.9 7.9 9 6.7 9S4.4 8 4.4 6.7s1-2.3 2.3-2.3S9 5.4 9 6.7zm16.3 21c-1.3 0-2.3-1-2.3-2.3s1-2.3 2.3-2.3 2.3 1 2.3 2.3-1 2.3-2.3 2.3zm2.4-21c0 1.3-1 2.3-2.3 2.3S23 7.9 23 6.7s1-2.3 2.3-2.3 2.4 1 2.4 2.3zM6.7 23C8 23 9 24 9 25.3s-1 2.3-2.3 2.3-2.3-1-2.3-2.3 1-2.3 2.3-2.3z"
                }
            }
        }
    }
}
