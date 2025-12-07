use dioxus::prelude::*;

#[component]
pub fn ThemeToggle() -> Element {
    rsx! {
        button {
            class: "outline",
            style: "border: none",
            "data-tooltip": "Toggle theme",
            "data-placement": "bottom",
            onclick: move |_| {
                let _ = document::eval(r#"
                    const html = document.documentElement;
                    const current = html.getAttribute("data-theme") ?? "dark";
                    const next = current === "dark" ? "light" : "dark";
                    html.setAttribute("data-theme", next);
                "#);
            },
            span { "☀️ / 🌙" }
        }
    }
}
