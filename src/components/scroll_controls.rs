use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons::FaArrowUp;

#[derive(Clone, Copy, Default, PartialEq)]
struct ScrollState {
    can_scroll: bool,
    show_back_to_top: bool,
    percentage: f64,
}

#[cfg(any(target_arch = "wasm32", test))]
impl ScrollState {
    fn from_dimensions(scroll_top: i32, scroll_height: i32, viewport_height: i32) -> Self {
        let max_scroll = scroll_height - viewport_height;
        // Ignore a single pixel of overflow caused by layout rounding.
        if max_scroll <= 1 {
            return Self::default();
        }

        Self {
            can_scroll: true,
            show_back_to_top: scroll_top > 1,
            percentage: (f64::from(scroll_top) / f64::from(max_scroll) * 100.0).clamp(0.0, 100.0),
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn update_scroll_state(mut state: Signal<ScrollState>) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Some(root) = document.document_element() else {
        return;
    };
    let scrolling_element = document.scrolling_element().unwrap_or_else(|| root.clone());
    let next = ScrollState::from_dimensions(
        scrolling_element.scroll_top(),
        scrolling_element.scroll_height(),
        root.client_height(),
    );

    if *state.peek() != next {
        state.set(next);
    }
}

#[cfg(target_arch = "wasm32")]
struct ScrollObserver {
    window: web_sys::Window,
    resize_observer: web_sys::ResizeObserver,
    on_change: wasm_bindgen::prelude::Closure<dyn FnMut()>,
}

#[cfg(target_arch = "wasm32")]
impl ScrollObserver {
    fn new(state: Signal<ScrollState>) -> Option<Self> {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::prelude::Closure;

        let window = web_sys::window()?;
        let document = window.document()?;
        let root = document.document_element()?;
        let on_change =
            Closure::wrap(Box::new(move || update_scroll_state(state)) as Box<dyn FnMut()>);
        let resize_observer =
            web_sys::ResizeObserver::new(on_change.as_ref().unchecked_ref()).ok()?;
        let observer = Self {
            window,
            resize_observer,
            on_change,
        };

        // The document and main container cover viewport, route, image, font,
        // and disclosure changes without observing our own progress updates.
        observer.resize_observer.observe(&root);
        if let Some(main) = document.query_selector("main.container").ok().flatten() {
            observer.resize_observer.observe(&main);
        }
        for event in ["scroll", "resize", "pageshow"] {
            observer
                .window
                .add_event_listener_with_callback(
                    event,
                    observer.on_change.as_ref().unchecked_ref(),
                )
                .ok()?;
        }

        update_scroll_state(state);
        Some(observer)
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for ScrollObserver {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;

        self.resize_observer.disconnect();
        for event in ["scroll", "resize", "pageshow"] {
            let _ = self.window.remove_event_listener_with_callback(
                event,
                self.on_change.as_ref().unchecked_ref(),
            );
        }
    }
}

#[component]
pub fn ScrollControls() -> Element {
    let scroll_state = use_signal(ScrollState::default);

    #[cfg(target_arch = "wasm32")]
    {
        let route = use_route::<crate::Route>();
        let mut observer = use_signal(|| None::<ScrollObserver>);

        use_effect(use_reactive((&route,), move |_| {
            // Replacing the observer also removes the previous route's listeners.
            observer.set(ScrollObserver::new(scroll_state));
        }));
    }

    let state = scroll_state();
    rsx! {
        if state.can_scroll {
            div {
                class: "scroll-progress-bar",
                style: "width: {state.percentage}%;",
                "aria-hidden": "true",
            }
            if state.show_back_to_top {
                button {
                    class: "back-to-top",
                    r#type: "button",
                    "aria-label": "back to top",
                    onclick: move |_| {
                        #[cfg(target_arch = "wasm32")]
                        if let Some(window) = web_sys::window() {
                            window.scroll_to_with_x_and_y(0.0, 0.0);
                        }
                    },
                    Icon { icon: FaArrowUp }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ScrollState;

    #[test]
    fn hides_controls_when_content_fits_with_rounding_tolerance() {
        for height in [600, 800, 801] {
            let state = ScrollState::from_dimensions(0, height, 800);
            assert!(!state.can_scroll);
            assert!(!state.show_back_to_top);
            assert_eq!(state.percentage, 0.0);
        }
        assert!(ScrollState::from_dimensions(0, 802, 800).can_scroll);
    }

    #[test]
    fn shows_back_to_top_only_after_leaving_top() {
        for (position, visible) in [(-20, false), (0, false), (1, false), (2, true), (400, true)] {
            let state = ScrollState::from_dimensions(position, 1600, 800);
            assert!(state.can_scroll);
            assert_eq!(state.show_back_to_top, visible);
        }
    }

    #[test]
    fn measures_progress_across_the_scrollable_range_and_clamps_overscroll() {
        for (position, expected) in [
            (-20, 0.0),
            (0, 0.0),
            (400, 50.0),
            (800, 100.0),
            (820, 100.0),
        ] {
            let state = ScrollState::from_dimensions(position, 1600, 800);
            assert!(state.can_scroll);
            assert_eq!(state.percentage, expected);
        }
    }
}
