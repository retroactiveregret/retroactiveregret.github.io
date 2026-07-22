use crate::{
    db::save_database, models::*,
};
use dioxus::prelude::*;

#[component]
pub fn UploadButton(db: Signal<Database>, status_message: Signal<Status>) -> Element {
    rsx!{
        input {
            id: "file-upload",
            class: "hidden",
            r#type: "file",

            onchange: move |evt| {
                async move {
                    for file in evt.files() {
                        match file.read_string().await {
                            Ok(text) => {
                                let database: DatabaseState = serde_json::from_str(&text)
                                    .unwrap();
                                match save_database(&database).await {
                                    Ok(_) => {
                                        status_message
                                            .write()
                                            .set_message(
                                                "Imported database successfully",
                                                StatusLevel::Success,
                                            )
                                    }
                                    Err(e) => {
                                        status_message
                                            .write()
                                            .set_message(
                                                format!("Error importing database: {:#?}", e),
                                                StatusLevel::Success,
                                            )
                                    }
                                }
                            }
                            Err(e) => {
                                status_message
                                    .write()
                                    .set_message(
                                        format!("Error reading file: {:#?}", e),
                                        StatusLevel::Success,
                                    )
                            }
                        }
                    }
                }
            },
        }

        label { class: "", role: "button", r#for: "file-upload", "Import .identi file" }
    }
}