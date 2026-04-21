use dioxus::prelude::*;

use crate::Route;
use crate::components::{GithubLink, LinkedinLink, ThemeToggle};

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            ul {
                li { Link { to: Route::Home {}, "Home" } }
                li { Link { to: Route::Blog {}, "Blog" } }
                li { Link { to: Route::Projects {}, "Projects" } }
            }
            ul {
                li { LinkedinLink {} }
                li { GithubLink {} }
                li { ThemeToggle {} }
            }
        }

        Outlet::<Route> {}
    }
}
