use dioxus::prelude::*;
use uuid::Uuid;
use crate::{Route, components::MemberAvatar, models::*};

#[component]
pub fn Fronters(
    db: Signal<Database>,
    status_message: Signal<Status>,
    fp: FrontPeriod,
) -> Element {
    let db = db();
    let members = db.members.read();

    let fronters: Vec<Member> = fp.assignments
        .iter()
        .filter_map(|assignment| members.get(&assignment.member_id))
        .cloned()
        .collect();

    rsx! {
        for member in fronters {
            div { class: "", role: "button",
                Link {
                    to: Route::EditFrontAssignment {
                        event_id: fp.id,
                        member_id: member.id,
                    },
                    MemberAvatar { img_id: member.avatar_asset_id, size: 24 }
                }
                div { class: "w-24 flex flex-row justify-center",
                    label { class: "label text-center text-ellipsis", "{member.name}" }
                }
            }
        }
    }
}
