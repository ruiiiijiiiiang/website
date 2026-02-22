use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaArrowUp;

#[component]
pub fn BackToTop() -> Element {
    rsx! {
        button {
            class: "back-to-top",
            onclick: move |_| {
                spawn(async move {
                    let _ = document::eval("window.scrollTo({ top: 0 })").await;
                });
            },
            Icon {
                icon: FaArrowUp
            }
        }
    }
}
