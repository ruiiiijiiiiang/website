use dioxus::prelude::*;

mod backend;
mod components;
mod models;
mod utils;
mod views;

use components::Navbar;
use views::{Blog, BlogPost, Home, Projects};

#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[redirect("/blog/", || Route::Blog {})]
        #[route("/blog")]
        Blog {},
        #[route("/blog/:slug")]
        BlogPost { slug: String },
        #[redirect("/projects/", || Route::Projects {})]
        #[route("/projects")]
        Projects {},
}

const FAVICON: Asset = asset!("../assets/favicon.ico");
const MAIN_CSS: Asset = asset!("../assets/main.css");
pub const DOMAIN: &str = "https://public.ruijiang.me";

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::server::{
            DioxusRouterExt, IncrementalRendererConfig, ServeConfig,
            axum::{self, Router, http::HeaderValue},
        };
        use tower_http::set_header::SetResponseHeaderLayer;

        let config = ServeConfig::builder()
            .incremental(
                IncrementalRendererConfig::new()
                    .static_dir(
                        std::env::current_exe()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .join("public"),
                    )
                    .clear_cache(false),
            )
            .enable_out_of_order_streaming();

        let app = Router::new()
            .serve_dioxus_application(config, App)
            .layer(SetResponseHeaderLayer::overriding(
                axum::http::header::STRICT_TRANSPORT_SECURITY,
                HeaderValue::from_static("max-age=63072000; includeSubDomains; preload"),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                axum::http::header::CONTENT_SECURITY_POLICY,
                HeaderValue::from_static(
                    "default-src 'self'; \
                     script-src 'self' 'unsafe-inline' 'unsafe-eval' 'wasm-unsafe-eval' https://static.cloudflareinsights.com; \
                     style-src 'self' 'unsafe-inline' https://cdn.jsdelivr.net https://fonts.googleapis.com; \
                     img-src 'self' data: https://raw.githubusercontent.com https://user-attachments.githubusercontent.com https://github.com https://github-production-user-asset-6210df.s3.amazonaws.com; \
                     font-src 'self' data: https://cdn.jsdelivr.net https://fonts.gstatic.com; \
                     connect-src 'self' ws: wss: https://cloudflareinsights.com; \
                     frame-ancestors 'none'; \
                     base-uri 'self'",
                ),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                axum::http::header::X_FRAME_OPTIONS,
                HeaderValue::from_static("DENY"),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                axum::http::header::X_CONTENT_TYPE_OPTIONS,
                HeaderValue::from_static("nosniff"),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                axum::http::header::REFERRER_POLICY,
                HeaderValue::from_static("strict-origin-when-cross-origin"),
            ))
            .layer(SetResponseHeaderLayer::overriding(
                axum::http::HeaderName::from_static("permissions-policy"),
                HeaderValue::from_static(
                    "camera=(), microphone=(), geolocation=()",
                ),
            ));

        Ok(app)
    });

    #[cfg(not(feature = "server"))]
    dioxus::LaunchBuilder::new().launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Fira+Mono:wght@400;500;700&family=Fira+Sans:ital,wght@0,300;0,400;0,500;0,700;1,300;1,400;1,500;1,700&display=swap",
        }
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
