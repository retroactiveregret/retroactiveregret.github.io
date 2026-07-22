use dioxus::prelude::*;

#[component]
pub fn Dev() -> Element {
    let mut route = use_signal(|| String::new());
    rsx! {
        div { class: "p-4 pb-0 text-xs opacity-60 tracking-wide", "Options" }
        ul { class: "list",
            li { class: "list-row",
                label { "Route to:" }
                input {
                    class: "input",
                    r#type: "text",
                    value: route(),
                    oninput: move |evt| route.set(evt.value()),
                }
                a { class: "btn", href: format!("/{}", route()), "Go" }
            }
        }
    }
}
