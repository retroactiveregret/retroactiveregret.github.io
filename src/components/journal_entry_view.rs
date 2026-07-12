use crate::{Route, components::Markdown, icons::*, models::*};
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn JournalEntryView(
    entry: JournalEntry,
    viewed_entry_id: Signal<Option<Uuid>>,
    date_entries: Memo<Option<Vec<Uuid>>>,
    viewed_position: Memo<Option<usize>>,
    show_post: Signal<bool>,
) -> Element {
    rsx! {
        div { class: "m-4",
            div { class: "flex flex-row justify-between",
                button {
                    onclick: move |_| {
                        if let (Some(ids), Some(pos)) = (date_entries(), viewed_position()) {
                            if pos > 0 {
                                viewed_entry_id.set(ids.get(pos - 1).cloned());
                            }
                        }
                    },
                    class: if viewed_position().is_some_and(|p| p > 0) { "label" } else { "invisible" },
                    Icon { data: material_symbols_light::ArrowBackIosRounded }
                }

                h1 { class: "text-center text-xl font-semibold", "{entry.title}" }

                button {
                    onclick: move |_| {
                        if let (Some(ids), Some(pos)) = (date_entries(), viewed_position()) {
                            if pos + 1 < ids.len() {
                                viewed_entry_id.set(ids.get(pos + 1).cloned());
                            }
                        }
                    },
                    class: if viewed_position().zip(date_entries()).is_some_and(|(p, ids)| p + 1 < ids.len()) { "label" } else { "invisible" },
                    Icon { data: material_symbols_light::ArrowForwardIosRounded }
                }
            }

            if entry.content_warning.is_some() && !show_post() {
                div { class: "flex justify-center items-center w-full h-[60vh]",
                    div { class: "foreground rounded-box p-5 w-full",
                        span { class: "inline-flex items-center gap-2 text-lg",
                            Icon {
                                class: "",
                                size: 32,
                                data: material_symbols_light::WarningOutlineRounded,
                            }

                            span { class: "text-xl font-semibold", "Content warning" }
                        }
                        p { class: "m-2", "{entry.content_warning.clone().unwrap()}" }
                        div { class: "flex justify-end w-full",
                            button {
                                class: "btn",
                                onclick: move |_| show_post.set(true),
                                "Show"
                            }
                        }
                    }
                }
            } else {
                div { class: "mt-7",
                    Markdown { text: "{entry.body}", class: "prose" }
                }
            }
        }
        div {
            class: "fab bottom-20 left-6 right-auto",
            tabindex: "0",
            role: "button",
            button { class: "btn w-12 h-12 p-0",
                Link { to: Route::JournalPost { id: entry.id },
                    Icon { size: 32, data: mdi_light::Pencil }
                }
            }
        }

        div { class: "fab bottom-20", tabindex: "0", role: "button",
            button { class: "btn w-12 h-12 p-0",
                Link { to: Route::AddJournalEntry {},
                    Icon { size: 32, data: material_symbols_light::AddRounded }
                }
            }
        }
    }
}
