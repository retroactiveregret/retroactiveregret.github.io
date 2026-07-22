use dioxus::prelude::*;
use uuid::Uuid;

use crate::{api::file_url, components::MemberAvatar, models::*};

#[component]
pub fn MemberList(db: Signal<Database>, on_click: Callback<Uuid>) -> Element {
    let settings = (db().settings)();
    rsx! {
        div { class: "p-4 text-xs opacity-60 tracking-wide", "Members" }
        ul { class: "list foreground rounded-xl shadow-md overflow-hidden m-4 mt-0",
            for (id , member) in db().members.read().iter().rev().filter(|(_, m)| !m.archived) {
                li {
                    class: "list-row flex items-center bg-cover bg-center rounded-none relative overflow-hidden",
                    background_image: format!("url({})", file_url(member.banner_asset_id.unwrap_or_default())),
                    onclick: {
                        let id = *id;
                        move |_| on_click.call(id)
                    },
                    div { class: "relative z-10",
                        MemberAvatar { img_id: member.avatar_asset_id, size: 16 }
                    }
                    div { class: "relative z-10",
                        div { class: "text-xl", "{member.name}" }
                    }
                    div {
                        class: format!(
                            "absolute inset-0 {}",
                            if settings.blur_banners {
                                "bg-neutral/50 backdrop-blur-sm"
                            } else {
                                "bg-neutral/70"
                            },
                        ),
                    }
                }
            }
        }
    }
}
