use dioxus::prelude::*;
use uuid::Uuid;

use crate::{api::image_url, icons::*};

#[component]
pub fn MemberAvatar(img_id: Option<Uuid>, size: usize) -> Element {
    match img_id {
        Some(asset) => rsx! {
            img {
                class: "size-[var(--s)] rounded-box foreground object-cover",
                style: format!("--s: {}px", size * 4),
                src: image_url(asset),
            }
        },
        None => rsx! {
            Icon {
                class: "rounded-box foreground bg-primary-content inset-ring-2 inset-ring-primary text-primary",
                size: size * 4,
                data: lucide::User,
                stroke_width: 4,
            }
        },
    }
}
