use dioxus::prelude::*;

mod backend;
mod components;
mod models;
mod utils;
mod views;

use components::Navbar;
use views::{Blog, Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: usize },
}

const FAVICON: Asset = asset!("../assets/favicon.ico");
const MAIN_CSS: Asset = asset!("../assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.slate.min.css" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        main {
            class: "container",
            Router::<Route> {}
        }
    }
}
