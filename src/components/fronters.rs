use dioxus::prelude::*;

use crate::{
    components::MemberAvatar,
    models::*,
};

#[component]
pub fn Fronters(
    db: Signal<Database>,
    status_message: Signal<Status>,
    assignments: Vec<FrontPeriodAssignment>,
    remove_callback: Callback<usize>,
) -> Element {
    let db = db();
    let members = db.members.read();

    let fronters: Vec<Member> = assignments
        .iter()
        .filter_map(|assignment| members.get(&assignment.member_id))
        .cloned()
        .collect();

    rsx! {
        for (i , member) in fronters.iter().enumerate() {
            div { class: "", role: "button",
                label { r#for: "remove-{member.id}",
                    MemberAvatar { img_id: member.avatar_asset_id, size: 24 }
                }
                div { class: "w-24 flex flex-row justify-center",
                    label { class: "label text-center text-ellipsis", "{member.name}" }
                }
            }

            input {
                class: "modal-toggle",
                id: "remove-{member.id}",
                r#type: "checkbox",
            }
            div { class: "modal", role: "dialog",
                div { class: "modal-box",
                    p { class: "py-4", "Remove {member.name} from front?" }
                    div { class: "flex flex-row justify-between",
                        label { class: "btn", r#for: "remove-{member.id}", "Cancel" }
                        button {
                            class: "btn",
                            onclick: move |_| remove_callback(i),
                            "Remove"
                        }
                    }
                }
                label { class: "modal-backdrop", r#for: "remove-{member.id}", "Close" }
            }
        }
    }
}
