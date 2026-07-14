use crate::{components::*, icons::*, models::*, Route};
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn EditFrontAssignment(event_id: Uuid, member_id: Uuid) -> Element {
    let mut db = use_context::<Signal<Database>>();
    let mut front_periods = (db().front_periods)();
    let members = (db().members)();

    let fp_mut = front_periods.get_mut(&event_id).unwrap();
    let assignment = fp_mut
        .to_owned()
        .assignments
        .iter()
        .find(|&a| a.member_id == member_id)
        .unwrap()
        .to_owned();
    let member = members.get(&member_id).unwrap();

    let front_role = use_signal(|| assignment.front_role);

    rsx! {
        div { class: "p-7",
            div { class: "w-full flex justify-center",
                MemberAvatar { img_id: member.avatar_asset_id, size: 48 }
            }
            FrontRoleDropdown { front_role }
        }
    }
}
