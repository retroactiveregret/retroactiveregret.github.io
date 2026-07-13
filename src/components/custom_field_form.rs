use dioxus::prelude::*;

use crate::models::*;

#[component]
pub fn CustomFieldForm(db: Signal<Database>, name_input: Signal<String>, on_click: Callback<()>) -> Element {
    rsx! {
        fieldset { class: "fieldset w-full",
            legend { class: "fieldset-legend", "Name" }
            div { class: "flex flex-row gap-2 w-full",
                input {
                    class: "input grow",
                    r#type: "text",
                    placeholder: "Name",
                    value: "{name_input}",
                    oninput: move |event| name_input.set(event.value()),
                }
                button { class: "btn", onclick: move |_| on_click(()), "Save" }
            }
        }
    }
}