use dioxus::prelude::*;

mod backend;
mod components;
mod models;
mod utils;
mod views;

use components::Navbar;
use views::{Blog, BlogPost, Home};

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/blog/")]
        Blog {},
        #[route("/blog/:slug")]
        BlogPost { slug: String },
}

const FAVICON: Asset = asset!("../assets/favicon.ico");
const MAIN_CSS: Asset = asset!("../assets/main.css");

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(server_only! {
            ServeConfig::builder()
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.slate.min.css",
            integrity: "sha384-hU2t8/WEbCrByoaMrUPwIkqtJDVUhoP5NbsnZzUOTKIiFp5CLBnUvP+LiLBR6tIw",
            crossorigin: "anonymous",
        }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Meta { name: "description", content: "Rui is a software engineer writing about Linux, NixOS, security, and systems design." }

        main {
            class: "container",
            Router::<Route> {}
        }
    }
}

#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    Ok(Route::static_routes()
        .iter()
        .map(ToString::to_string)
        .collect())
}
