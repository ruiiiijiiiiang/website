use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaArrowLeft, FaArrowRight};

use crate::Route;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            nav {
                ul {
                    li {
                        Link {
                            to: Route::Blog { id: 0 },
                            Icon {
                                icon: FaArrowLeft
                            }
                        }
                    }
                }
                ul {
                    li {
                        Link {
                            to: Route::Blog { id: 0 },
                            Icon {
                                icon: FaArrowRight
                            }
                        }
                    }
                }
            }
        }
    }
}
