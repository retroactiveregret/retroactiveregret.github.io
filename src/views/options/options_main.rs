use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Options() -> Element {
    rsx! {
        div { class: "p-4 pb-0 text-xs opacity-60 tracking-wide", "Options" }
        ul { class: "list",
            li { class: "list-row",
                Link { to: Route::Appearance {}, "Appearance" }
            }
            li { class: "list-row",
                Link { to: Route::CustomFields {}, "Custom Fields" }
            }
            li { class: "list-row",
                Link { to: Route::FrontHistory {}, "Front History" }
            }
            li { class: "list-row",
                Link { to: Route::BoardHistory {}, "Board History" }
            }
            li { class: "list-row",
                Link { to: Route::Security {}, "Security" }
            }
            li { class: "list-row",
                Link { to: Route::ImportExport {}, "Import/Export" }
            }
            li { class: "list-row",
                Link { to: Route::About {}, "About" }
            }
        }
    }
}