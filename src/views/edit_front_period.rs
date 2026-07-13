use chrono::NaiveDate;
use chrono::Local;
use dioxus::prelude::*;
use uuid::Uuid;
use crate::Route;
use crate::api::local_naive_to_utc;
use crate::api::put_front_period;
use crate::api::time_format;
use crate::icons::*;
use crate::models::*;
use crate::components::*;

#[component]
pub fn EditFrontPeriod(id: Uuid) -> Element {
    let db = use_context::<Signal<Database>>();
    let twelve_hour = (db().settings)().twelve_hour;
    let mut status_message = use_context::<Signal<Status>>();
    let mut show_select = use_signal(|| false);

    let front_periods = (db().front_periods)();
    let fp = front_periods.get(&id).unwrap().clone();

    let mut assignments = use_signal(|| fp.assignments.clone());
    let mut started_at = use_signal(|| fp.started_at.clone());
    let mut ended_at = use_signal(|| fp.ended_at.unwrap().clone());

    // Convert UTC -> Local for display/editing
    let mut start_date_part = use_signal(|| started_at().with_timezone(&Local).date_naive());
    let mut start_time_part = use_signal(|| started_at().with_timezone(&Local).time());

    let mut end_date_part = use_signal(|| ended_at().with_timezone(&Local).date_naive());
    let mut end_time_part = use_signal(|| ended_at().with_timezone(&Local).time());

    let save = move |_| {
        let start_naive = start_date_part().and_time(start_time_part());
        let end_naive = end_date_part().and_time(end_time_part());

        started_at.set(local_naive_to_utc(start_naive));
        ended_at.set(local_naive_to_utc(end_naive));

        match put_front_period(fp.id, started_at(), ended_at(), assignments(), String::new()) {
            Ok(_) => {}
            Err(err) => status_message.write().set_message(
                format!("Error editing front period: {:#?}", err),
                StatusLevel::Error,
            ),
        }
        show_select.set(false);
        navigator().push(Route::Dashboard {});
    };

    let add_member = move |id: Uuid| {
        assignments.push(FrontPeriodAssignment { member_id: id });
        show_select.set(false);
    };

    let remove_callback = move |i: usize| {
        assignments.remove(i);
    };

    rsx! {
        div { class: "flex flex-col p-7 gap-4",
            div { class: "flex flex-row gap-4 overflow-x-scroll items-start",
                div { class: "h-24 w-24",
                    button {
                        class: "btn w-24 h-24 foreground grow rounded-box flex justify-center items-center p-0",
                        onclick: move |_| show_select.set(true),
                        Icon {
                            size: 64,
                            data: material_symbols_light::AddRounded,
                        }
                    }
                }
                Fronters {
                    db,
                    status_message,
                    assignments: assignments(),
                    remove_callback,
                }
            }

            div { class: "flex flex-row gap-2 grow",
                div { class: "basis-1/2 flex flex-col gap-2 items-center p-2",
                    h2 { class: "text-2xl text-center", "Start time" }
                    input {
                        class: "w-fit",
                        r#type: "date",
                        value: start_date_part().format("%F").to_string(),
                        oninput: move |evt| start_date_part.set(evt.value().parse::<NaiveDate>().unwrap()),
                    }
                    input {
                        class: "w-fit",
                        r#type: "time",
                        value: start_time_part().format("%H:%M").to_string(),
                        oninput: move |evt| {
                            start_time_part
                                .set(chrono::NaiveTime::parse_from_str(&evt.value(), "%H:%M").unwrap());
                        },
                    }
                }

                div { class: "basis-1/2 flex flex-col gap-2 items-center p-2",
                    h2 { class: "text-2xl text-center", "End time" }
                    input {
                        class: "w-fit",
                        r#type: "date",
                        value: end_date_part().format("%F").to_string(),
                        oninput: move |evt| end_date_part.set(evt.value().parse::<NaiveDate>().unwrap()),
                    }
                    input {
                        class: "w-fit",
                        r#type: "time",
                        value: end_time_part().format("%H:%M").to_string(),
                        oninput: move |evt| {
                            end_time_part
                                .set(chrono::NaiveTime::parse_from_str(&evt.value(), "%H:%M").unwrap());
                        },
                    }
                }
            }

            div { class: "flex flex-row justify-between w-full",
                button { class: "btn", onclick: move |_| navigator().go_back(), "Cancel" }
                button { class: "btn btn-primary", onclick: save, "Save" }
            }
        }

        if show_select() {
            div { class: "w-screen h-full fixed inset-0 bg-base-100 z-1 m-0",
                MemberList { db, on_click: add_member }
            }
        }
    }
}