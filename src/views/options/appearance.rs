use dioxus::prelude::*;
use crate::{Route, models::Database};

#[component]
pub fn Appearance() -> Element {
    let db = use_context::<Signal<Database>>();
    let mut settings = db().settings;
    
    rsx! {
        div { class: "p-4 pb-0 text-xs opacity-60 tracking-wide", "Appearance" }
        ul { class: "list",
            li { class: "list-row",
                Link { class: "", to: Route::Theme {}, "Theme" }
            }
            li { class: "list-row gap-2",
                p { class: "", "Blur member banners" }
                div { class: "list-col-wrap ",
                    input {
                        class: "toggle",
                        r#type: "checkbox",
                        checked: settings().blur_banners,
                        oninput: move |evt| {
                            if evt.value().parse().unwrap_or(false) {
                                settings.write().blur_banners = true;
                            } else {
                                settings.write().blur_banners = false;
                            }
                        },
                    }
                }
            }
            li { class: "list-row gap-2",
                p { class: "", "Unread notification indicator" }
                div { class: "list-col-wrap ",
                    span { class: "label text-xs", "Background color" }
                    input {
                        class: "toggle toggle-primary ml-2 mr-2",
                        r#type: "checkbox",
                        checked: settings().outline_notifications,
                        oninput: move |evt| {
                            if evt.value().parse().unwrap_or(false) {
                                settings.write().outline_notifications = true;
                            } else {
                                settings.write().outline_notifications = false;
                            }
                        },
                    }
                    span { class: "label text-xs", "Outline" }
                }
            }
            li { class: "list-row gap-2",
                p { class: "", "Notification pop-up" }
                div { class: "list-col-wrap ",
                    input {
                        class: "toggle",
                        r#type: "checkbox",
                        checked: settings().notification_popup,
                        oninput: move |evt| {
                            if evt.value().parse().unwrap_or(false) {
                                settings.write().notification_popup = true;
                            } else {
                                settings.write().notification_popup = false;
                            }
                        },
                    }
                }
            }
            li { class: "list-row gap-2",
                p { class: "", "Notification banner" }
                div { class: "list-col-wrap ",
                    input {
                        class: "toggle",
                        r#type: "checkbox",
                        checked: settings().notification_banner,
                        oninput: move |evt| {
                            if evt.value().parse().unwrap_or(false) {
                                settings.write().notification_banner = true;
                            } else {
                                settings.write().notification_banner = false;
                            }
                        },
                    }
                }
            }
        }
    }
}