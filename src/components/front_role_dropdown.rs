use crate::models::*;
use dioxus::prelude::*;

#[component]
pub fn FrontRoleDropdown(front_role: Signal<FrontRole>) -> Element {
    info!("Front role: {}", front_role().to_string());
    rsx! {
        select {
            class: "select w-full",
            oninput: move |evt| front_role.set(evt.value().into()),

            option {
                disabled: "true",
                label: "Unspecified",
                value: "unknown",
                selected: front_role() == FrontRole::Unknown,
            }
            option {
                label: "Primary",
                value: "primary",
                selected: front_role() == FrontRole::Primary,
            }
            option {
                label: "Co-front",
                value: "cofront",
                selected: front_role() == FrontRole::CoFront,
            }
            option {
                label: "Co-conscious",
                value: "cocon",
                selected: front_role() == FrontRole::CoCon,
            }
            option {
                label: "Influencing",
                value: "influencing",
                selected: front_role() == FrontRole::Influencing,
            }
        }
    }
}
