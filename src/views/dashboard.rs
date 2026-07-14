use crate::{api, components::*, models::*};
use chrono::Utc;
use dioxus::prelude::*;

pub fn Dashboard() -> Element {
    let db = use_context::<Signal<Database>>();
    let mut status_message = use_context::<Signal<Status>>();
    let active = use_memo(move || {
        info!("Memo");
        db().get_active_period()
    });
    info!("Active: {:#?}", active());

    rsx! {
        div { class: "flex flex-row gap-4 p-4 overflow-x-scroll",
            Switch { db, status_message }
            match active() {
                Some(fp) => rsx! {
                    Fronters { db, status_message, fp }
                },
                None => rsx! {},
            }
        }
        BoardPosts { db, status_message }
        ul { class: "list m-4 mt-0 foreground rounded-box shadow-md",
            li { class: "p-4 pb-2 text-xs opacity-60 tracking-wide", "Front history" }
            FrontHistoryList {
                db,
                status_message,
                history: (db().front_periods)(),
                max_show: (db().settings)().front_history_show,
            }
        }
    }
}
