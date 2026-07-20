use crate::{components::MemberAvatar, models::*, Route};
use dioxus::prelude::*;

#[component]
pub fn Fronters(db: Signal<Database>, status_message: Signal<Status>, fp: FrontPeriod) -> Element {
    let binding = db();
    let members = binding.members.read();
    let subsystems = binding.subsystems.read();

    let fronters: Vec<Member> = fp
        .assignments
        .iter()
        .filter_map(|assignment| {
            members.get(&assignment.member_id).cloned().or(Some(
                subsystems
                    .get(&assignment.member_id)
                    .unwrap()
                    .to_member(db()),
            ))
        })
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
