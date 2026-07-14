use chrono::Utc;
use dioxus::prelude::*;
use uuid::Uuid;

use crate::{api::switch, components::MemberList, icons::*, models::*};

#[component]
pub fn Switch(db: Signal<Database>, status_message: Signal<Status>) -> Element {
    let mut show_select_swap = use_signal(|| false);
    let mut show_select_add = use_signal(|| false);

    let swap_on_click = move |uuid: Uuid| {
        let assignments = vec![FrontPeriodAssignment {
            member_id: uuid,
            front_role: FrontRole::Unknown,
            confidence: 1.0,
            note: String::new(),
        }];
        match switch(Utc::now(), assignments) {
            Ok(_) => {}
            Err(err) => status_message.write().set_message(
                format!("Error adding member: {:#?}", err),
                StatusLevel::Error,
            ),
        }
        show_select_swap.set(false);
    };

    let add_on_click = move |uuid: Uuid| {
        let assignments = match db().get_active_period() {
            Some(active) => {
                let mut new = active.assignments.clone();
                new.push(FrontPeriodAssignment {
                    member_id: uuid,
                    front_role: FrontRole::Unknown,
                    confidence: 1.0,
                    note: String::new(),
                });
                new
            }
            None => vec![FrontPeriodAssignment {
                member_id: uuid,
                front_role: FrontRole::Unknown,
                confidence: 1.0,
                note: String::new(),
            }],
        };
        match switch(Utc::now(), assignments) {
            Ok(_) => {}
            Err(err) => status_message.write().set_message(
                format!("Error adding member: {:#?}", err),
                StatusLevel::Error,
            ),
        }
        show_select_add.set(false);
    };

    rsx! {
        div { class: "h-24 w-24 flex flex-col",
            button {
                class: "w-24 btn grow mb-2 rounded-box flex justify-center items-center p-0",
                onclick: move |_| show_select_swap.set(true),
                Icon { size: 32, data: material_symbols_light::SwapHorizRounded }
            }
            button {
                class: "w-24 btn grow mt-2 rounded-box flex justify-center items-center p-0",
                onclick: move |_| show_select_add.set(true),
                Icon { size: 32, data: material_symbols_light::AddRounded }
            }
        }

        if show_select_swap() {
            div { class: "w-screen h-full fixed inset-0 bg-base-100 z-1 m-0",
                MemberList { db, on_click: swap_on_click }
            }
        } else if show_select_add() {
            div { class: "w-screen h-full fixed inset-0 bg-base-100 z-1 m-0",
                MemberList { db, on_click: add_on_click }
            }
        }
    }
}
