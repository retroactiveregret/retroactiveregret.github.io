use crate::{Route, components::*, icons::*, models::*};
use chrono::Utc;
use dioxus::prelude::*;

#[component]
pub fn Journal() -> Element {
    let db = use_context::<Signal<Database>>();
    let date_map = use_context::<Memo<JournalDates>>();

    let mut show_post = use_signal(|| false);
    let date = use_signal(|| Utc::now().date_naive());
    let date_entries = use_memo(move || date_map().get(&date()).cloned());

    info!("{:#?}", date_map());
    info!("{:#?}", date());
    info!("{:#?}", date_entries());

    let mut viewed_entry_id = use_signal(|| None::<uuid::Uuid>);

    use_effect(move || {
        viewed_entry_id.set(date_entries().and_then(|ids| ids.last().cloned()));
    });

    let viewed_entry = use_memo(move || {
        viewed_entry_id().and_then(|id| db().journal_entries.read().get(&id).cloned())
    });

    let viewed_position = use_memo(move || {
        let ids = date_entries()?;
        let id = viewed_entry_id()?;
        ids.iter().position(|&entry_id| entry_id == id)
    });

    use_effect(move || {
        let _ = viewed_entry_id();
        show_post.set(false);
    });

    info!("{:#?}", viewed_entry_id());

    rsx! {
        DateSelect { date }
        div { class: "m-7",
            match viewed_entry() {
                Some(entry) => rsx! {
                    JournalEntryView {
                        entry,
                        viewed_entry_id,
                        date_entries,
                        viewed_position,
                        show_post,
                    }
                },
                None => rsx! {
                    p { "No entries for date" }
                },
            }
        }

        div { class: "fab bottom-20",
            button { class: "btn w-12 h-12 p-0",
                Link { to: Route::AddJournalEntry {},
                    Icon { size: 32, data: material_symbols_light::AddRounded }
                }
            }
        }
    }
}
