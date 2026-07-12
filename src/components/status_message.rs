use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::models::*;

#[component]
pub fn StatusMessage(message: Signal<Status>) -> Element {
    let mut show = use_signal(|| message().display_time_check());

    if show() {
        use_effect(move || {
            message();
            show.set(true);
            let timeout = match message().level {
                StatusLevel::Error => 5000,
                _ => 500,
            };
            spawn(async move {
                let _timer = TimeoutFuture::new(timeout).await;
                show.set(false);
            });
        });

        rsx! {
            div { class: "w-full p-5 fixed z-50",
                div {
                    class: format!("w-full alert {}", message().alert_class()),
                    role: "alert",
                    p { "{message()}" }
                }
            }
        }
    } else {
        rsx! {}
    }
}
