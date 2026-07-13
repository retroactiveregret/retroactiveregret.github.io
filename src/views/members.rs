use dioxus::prelude::*;
use uuid::Uuid;

use crate::{components::*, icons::*, models::*, Route};

pub fn Members() -> Element {
    let db = use_context::<Signal<Database>>();

    rsx! {
        div {
            MemberList {
                db,
                on_click: move |id: Uuid| {
                    use_navigator().push(Route::EditMember { id });
                },
            }
            div { class: "fab bottom-20",
                button { class: "btn w-12 h-12 p-0",
                    Link { to: Route::AddMember {},
                        Icon {
                            size: 32,
                            data: material_symbols_light::AddRounded,
                        }
                    }
                }
            }
        }
    }
}
