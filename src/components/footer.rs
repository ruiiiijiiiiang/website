use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::{FaArrowLeft, FaArrowRight};

use crate::Route;

#[component]
pub fn Footer(current: usize, count: usize) -> Element {
    rsx! {
        footer {
            nav {
                ul {
                    li {
                        if count != 0 && current != 0 {
                            Link {
                                to: Route::Blog { id: if current > 0 { current - 1 } else { 0 } },
                                Icon {
                                    icon: FaArrowLeft
                                }
                            }
                        }
                    }
                }
                ul {
                    li {
                        if count != 0 && current != count - 1 {
                            Link {
                                to: Route::Blog { id: current + 1 },
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
}
