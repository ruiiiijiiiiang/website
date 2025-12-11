use dioxus::prelude::*;

use crate::Route;
use crate::components::{GithubLink, ThemeToggle};

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            ul {
                li { Link { to: Route::Home {}, "Home" } }
                li { Link { to: Route::Blog { id: 0 }, "Blog" } }
            }
            ul {
                li { ThemeToggle {} }
                li { GithubLink {} }
            }
        }

        Outlet::<Route> {}
    }
}
