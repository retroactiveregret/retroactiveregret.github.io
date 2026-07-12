use dioxus::prelude::*;
use crate::components::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "p-7 text-center py-18",
            h1 { class: "text-xl", "Identi" }
            p { "Pre-release" }
        }
    }
}