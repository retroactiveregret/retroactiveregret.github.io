use dioxus::prelude::*;

#[component]
pub fn ComingSoon() -> Element {
    rsx! {
        div { class: "p-7 text-center w-full",
            h1 { class: "text-3xl font-semibold", "Coming soon..." }
        }
    }
}