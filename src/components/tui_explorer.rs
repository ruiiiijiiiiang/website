use std::{cell::RefCell, collections::HashMap, rc::Rc};

use dioxus::prelude::*;

const EXPLORER_CSS: Asset = asset!("../../assets/tui_explorer.css");

/// A stable, unique ID, a navigation label, and arbitrary title/detail content.
#[derive(Clone, PartialEq)]
pub struct ExplorerEntry {
    pub id: String,
    pub label: String,
    /// Optional detail title content. When absent, the navigation label is shown.
    pub title: Option<Element>,
    pub content: Element,
}

/// A content-agnostic master/detail explorer. `id` must be unique on the page;
/// entry IDs must be unique within this instance and stable across updates.
/// The first entry is selected initially, or if the selected entry is removed.
/// Navigation shortcuts apply only while focus is inside the entry list.
#[component]
pub fn TuiExplorer(
    id: String,
    label: String,
    entries: Vec<ExplorerEntry>,
    #[props(default = "No entries to display.".to_string())] empty_message: String,
) -> Element {
    let mut selected_id = use_signal(|| entries.first().map(|entry| entry.id.clone()));
    let buttons = use_hook(|| Rc::new(RefCell::new(HashMap::<String, Rc<MountedData>>::new())));
    let selected_index = entries
        .iter()
        .position(|entry| selected_id.read().as_deref() == Some(entry.id.as_str()))
        .unwrap_or(0);
    let selected = entries.get(selected_index);
    let previous = entries
        .get(selected_index.saturating_sub(1))
        .map(|entry| entry.id.clone());
    let next = entries
        .get((selected_index + 1).min(entries.len().saturating_sub(1)))
        .map(|entry| entry.id.clone());
    let keyboard_buttons = buttons.clone();
    let count = entries.len();
    let position = if selected.is_some() {
        selected_index + 1
    } else {
        0
    };

    rsx! {
        document::Link { rel: "stylesheet", href: EXPLORER_CSS }
        section { class: "tui-explorer", "aria-label": "{label} explorer",
            div { class: "tui-explorer-panes",
                aside { class: "tui-explorer-sidebar", "aria-label": "{label}",
                    div { class: "tui-explorer-title", "{label}/" }
                    ul {
                        class: "tui-explorer-list",
                        onkeydown: move |event: KeyboardEvent| {
                            if !event.modifiers().is_empty() || event.is_composing() {
                                return;
                            }
                            let target = match event.key() {
                                Key::Character(key) if key == "j" => next.clone(),
                                Key::Character(key) if key == "k" => previous.clone(),
                                _ => return,
                            };
                            if let Some(target) = target {
                                event.prevent_default();
                                event.stop_propagation();
                                selected_id.set(Some(target.clone()));
                                let button = keyboard_buttons.borrow().get(&target).cloned();
                                if let Some(button) = button {
                                    spawn(async move {
                                        let _ = button.set_focus(true).await;
                                    });
                                }
                            }
                        },
                        for (index, entry) in entries.iter().enumerate() {
                            li { key: "{entry.id}",
                                button {
                                    r#type: "button",
                                    class: "tui-explorer-entry",
                                    "aria-pressed": if index == selected_index { "true" } else { "false" },
                                    "aria-controls": "{id}-detail",
                                    onmounted: {
                                        let buttons = buttons.clone();
                                        let entry_id = entry.id.clone();
                                        move |event| {
                                            buttons.borrow_mut().insert(entry_id.clone(), event.data());
                                        }
                                    },
                                    onclick: {
                                        let entry_id = entry.id.clone();
                                        move |_| selected_id.set(Some(entry_id.clone()))
                                    },
                                    span { class: "tui-explorer-marker", "aria-hidden": "true",
                                        if index == selected_index { "▸" } else { "·" }
                                    }
                                    span { class: "tui-explorer-entry-label", "{entry.label}" }
                                }
                            }
                        }
                    }
                }
                section {
                    id: "{id}-detail",
                    class: "tui-explorer-detail",
                    "aria-label": selected.map(|entry| entry.label.as_str()).unwrap_or("Empty"),
                    div { id: "{id}-detail-title", class: "tui-explorer-title",
                        if let Some(entry) = selected {
                            div { key: "{entry.id}", class: "tui-explorer-title-content",
                                if let Some(title) = &entry.title {
                                    {title.clone()}
                                } else {
                                    "{entry.label}"
                                }
                            }
                        } else {
                            "Empty"
                        }
                    }
                    div { class: "tui-explorer-content",
                        if let Some(entry) = selected {
                            // Remount on selection so local state (such as open previews)
                            // belongs to the selected entry rather than the prior entry.
                            div { key: "{entry.id}", {entry.content.clone()} }
                        } else {
                            p { class: "tui-explorer-empty", "{empty_message}" }
                        }
                    }
                }
            }
            div { class: "tui-explorer-status",
                span { class: "tui-explorer-position", "{position}/{count}" }
                span { class: "tui-explorer-hint", kbd { "j" } " / " kbd { "k" } " navigate · click to select" }
            }
        }
    }
}
