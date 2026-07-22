use crate::{
    api::{file_url, upload},
    models::*,
};
use chrono::Utc;
use dioxus::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::Blob;

#[component]
pub fn DownloadButton(db: Signal<Database>, status_message: Signal<Status>) -> Element {
    let href_res = use_resource(move || async move {
        let db_state: DatabaseState = db().into();
        let json = serde_json::to_string_pretty(&db_state).unwrap();
        let blob = Blob::from(JsValue::from_str(&json));
        let id = upload(blob).await.unwrap();
        file_url(id)
    });

    match &*href_res.read_unchecked() {
        Some(href) => rsx! {
            a {
                class: "",
                href,
                download: format!("{}.identi", Utc::now().format("%Y_%m_%d_%H_%M_%S")),
                "Export database"
            }
        },
        None =>  rsx! {}
    }
}
