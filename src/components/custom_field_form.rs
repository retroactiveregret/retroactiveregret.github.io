use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::*;

#[component]
pub fn CustomFieldForm(db: Signal<Database>, name_input: Signal<String>, on_click: Callback<()>) -> Element {
    rsx! {
        fieldset { class: "fieldset",
            legend { class: "fieldset-legend", "Name" }
            div { class: "flex flex-row gap-2",
                input {
                    class: "input basis-3/4",
                    r#type: "text",
                    placeholder: "Name",
                    value: "{name_input}",
                    oninput: move |event| name_input.set(event.value()),
                }
                button { class: "btn basis-1/4", onclick: move |_| on_click(()), "Save" }
            }
        }
    }
}