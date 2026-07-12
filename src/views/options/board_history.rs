use chrono::Utc;
use dioxus::prelude::*;
use crate::{components::*, models::*};

#[component]
pub fn BoardHistory() -> Element {
    let db = use_context::<Signal<Database>>();
    let board_posts = (db().board_posts)();
    let status_message = use_context::<Signal<Status>>();
    let date_map = use_context::<Memo<PostDates>>();

    let date = use_signal(|| Utc::now().date_naive());
    let date_entries = use_memo(move || date_map().get(&date()).cloned());

    rsx! {
        DateSelect { date }

        if let Some(entries) = date_entries() {
            div { class: "w-full flex flex-col gap-2 p-2",
                for entry in entries {
                    Post {
                        db,
                        status_message,
                        post: board_posts.get(&entry).unwrap().to_owned(),
                        show_icons: false,
                    }
                }
            }
        }
    }
}