use crate::{components::*, models::*};
use chrono::Utc;
use dioxus::prelude::*;

#[component]
pub fn FrontHistory() -> Element {
    let db = use_context::<Signal<Database>>();
    let front_periods = (db().front_periods)();
    let status_message = use_context::<Signal<Status>>();
    let date_map = use_context::<Memo<SwitchDates>>();

    let date = use_signal(|| Utc::now().date_naive());
    let date_entries = use_memo(move || date_map().get(&date()).cloned());

    rsx! {
        DateSelect { date }

        if let Some(entries) = date_entries() {
            ul { class: "m-4 list",
                FrontHistoryList {
                    db,
                    status_message,
                    history: entries.iter().map(|e| (*e, front_periods.get(e).unwrap().to_owned())).collect(),
                    max_show: usize::MAX,
                }
            }
        }
    }
}
