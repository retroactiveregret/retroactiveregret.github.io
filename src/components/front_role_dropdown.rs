use crate::models::*;
use dioxus::prelude::*;

#[component]
pub fn FrontRoleDropdown(front_role: Signal<FrontRole>) -> Element {
    rsx! {
        select {
            class: "select",
            oninput: move |evt| front_role.set(evt.value().into()),

            option { disabled: "true", "Unspecified" }
            option { label: "Primary", value: "primary" }
            option { label: "Co-front", value: "cofront" }
            option { label: "Co-conscious", value: "cocon" }
            option { label: "Influencing", value: "influencing" }
        }
    }
}
