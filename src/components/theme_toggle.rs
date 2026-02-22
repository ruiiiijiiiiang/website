use dioxus::prelude::*;

const THEME_TOGGLE_CSS: Asset = asset!("../../assets/theme_toggle.css");
const DEFAULT_THEME: &str = "dark";

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme = use_signal(|| DEFAULT_THEME.to_string());
    rsx! {
        link { rel: "stylesheet", href: THEME_TOGGLE_CSS }
        button {
            class: "outline",
            style: "border: none; padding: 0;",
            "data-tooltip": "Toggle theme",
            "data-placement": "bottom",
            onclick: move |_| {
                spawn(async move {
                    let js_code = format!(r#"
                        const html = document.documentElement;
                        const current = html.getAttribute("data-theme") ?? "{}";
                        const next = current === "dark" ? "light" : "dark";
                        html.setAttribute("data-theme", next);
                        return next;
                    "#, DEFAULT_THEME);
                    let next_theme = document::eval(&js_code).await.unwrap().as_str().unwrap_or(DEFAULT_THEME).to_string();
                    theme.set(next_theme);
                });
            },
            ThemeIcon { moon: theme() == "dark" }
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
