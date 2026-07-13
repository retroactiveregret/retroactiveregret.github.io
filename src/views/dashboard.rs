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

    let remove_callback = move |i: usize| {
        let assignments = active.unwrap().assignments;
        if assignments.len() > 1 {
            let mut new = assignments;
            new.remove(i);
            match api::switch(Utc::now(), new, String::new()) {
                Ok(_) => {}
                Err(err) => status_message.write().set_message(
                    format!("Error removing member: {:#?}", err),
                    StatusLevel::Error,
                ),
            }
        } else {
            match api::end_current_period(Utc::now()) {
                Ok(_) => {}
                Err(err) => status_message.write().set_message(
                    format!("Error removing member: {:#?}", err),
                    StatusLevel::Error,
                ),
            }
        }
    };

    rsx! {
        div { class: "flex flex-row gap-4 p-4 overflow-x-scroll",
            Switch { db, status_message }
            match active() {
                Some(fp) => rsx! {
                    Fronters {
                        db,
                        status_message,
                        assignments: fp.assignments,
                        remove_callback,
                    }
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
