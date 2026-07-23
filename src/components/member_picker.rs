use dioxus::prelude::*;
use uuid::Uuid;
use crate::{components::MemberList, models::Database};

#[component]
pub fn MemberPicker(
    db: Signal<Database>,
    show_select: Signal<bool>,
    on_click: Callback<Uuid>,
) -> Element {
    if show_select() {
        rsx! {
            div { class: "w-full h-full fixed bg-base-100 z-1 inset-0",
                MemberList { db, on_click }
            }
        }
    } else {
        rsx! {}
    }
}