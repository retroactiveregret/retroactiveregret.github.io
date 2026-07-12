use crate::models::*;
use dioxus::prelude::*;

const LIGHT_THEMES: [&str; 20] = [
    "Cupcake",
    "Bumblebee",
    "Emerald",
    "Corporate",
    "Retro",
    "Cyberpunk",
    "Valentine",
    "Garden",
    "Lofi",
    "Pastel",
    "Fantasy",
    "Wireframe",
    "CMYK",
    "Autumn",
    "Acid",
    "Lemonade",
    "Winter",
    "Nord",
    "Caramellatte",
    "Silk",
];
const DARK_THEMES: [&str; 13] = [
    "Synthwave",
    "Halloween",
    "Forest",
    "Aqua",
    "Black",
    "Luxury",
    "Dracula",
    "Business",
    "Night",
    "Coffee",
    "Dim",
    "Sunset",
    "Abyss",
];

#[component]
pub fn Theme() -> Element {
    let db = use_context::<Signal<Database>>();
    let oninput = move |evt: Event<FormData>| {
        let mut settings = (db().settings)();
        settings.theme = evt.value();
        db().settings.set(settings);
        info!("Set theme to {}", evt.value());
    };

    rsx! {
        div { class: "p-7 flex flex-row",
            fieldset { class: "fieldset basis-1/2",
                label { class: "label", "Light mode" }

                for theme in LIGHT_THEMES {
                    label { class: "flex gap-2 cursor-pointer items-center",
                        input {
                            class: "radio radio-sm theme-controller",
                            name: "theme-radios",
                            r#type: "radio",
                            value: theme.to_lowercase(),
                            oninput,
                        }
                        "{theme}"
                    }
                }
            }
            fieldset { class: "fieldset basis-1/2",
                label { class: "label", "Dark mode" }

                for theme in DARK_THEMES {
                    label { class: "flex gap-2 cursor-pointer items-center",
                        input {
                            class: "radio radio-sm theme-controller",
                            name: "theme-radios",
                            r#type: "radio",
                            value: theme.to_lowercase(),
                            oninput,
                        }
                        "{theme}"
                    }
                }
            }
        }
    }
}
