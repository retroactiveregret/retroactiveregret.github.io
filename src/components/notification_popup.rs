use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use uuid::Uuid;

use crate::models::*;

#[component]
pub fn NotificationPopup(
    db: Signal<Database>,
    status_message: Signal<Status>,
    user_mentions_lookup: Memo<UserMentionsLookup>,
) -> Element {
    if (db().settings)().notification_popup {
        let mut showed_notification = use_context::<Signal<bool>>();
        let fronters = use_context::<Memo<Option<Vec<Uuid>>>>();
        
        let mentioned_users = use_memo(move || {
            info!("Running notifications memo");
            let mut m = Vec::<Member>::new();
            match fronters() {
                Some(f) => {
                    for id in f {
                        if let Some(notifs) = user_mentions_lookup().get(&id) {
                            if notifs
                                .iter()
                                .filter(|&n| !db().user_mentions.read()[n].read)
                                .next()
                                .is_some()
                            {
                                m.push(db().members.read()[&id].clone());
                            }
                        }
                    }
                }
                None => {}
            }
            m
        });

        use_effect(move || {
            let _ = mentioned_users();
            showed_notification.set(false);
            spawn(async move {
                let _timer = TimeoutFuture::new(2500).await;
                showed_notification.set(true);
            });
        });

        info!("Fronters: {:#?}", fronters());
        info!("Mentioned users: {:#?}", mentioned_users());
        info!("All notifications: {:#?}", db().user_mentions);

        rsx! {
            div { class: "w-full p-5 fixed z-50",
                for member in mentioned_users() {
                    if !showed_notification() {
                        div {
                            class: "w-full alert alert-primary alert-info alert-soft",
                            role: "alert",
                            "New notification for {member.name}"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
