use dioxus::prelude::*;

use crate::{components::{GoHome, INPUT_CLASSES}, utils::ART1Config};

#[component]
pub fn ART1Page() -> Element {
    let mut input_file_path = use_signal(|| "".to_string());
    

    rsx! {
        div {
            class: "flex flex-col gap-4",
            GoHome {  }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
            h3 {
                class: "text-gray-900 dark:text-white text-lg font-semibold",
                "База данных"
            }
            form {
                class: "contents",
                label { 
                    for: "input_file",
                    class: INPUT_CLASSES,
                    if input_file_path.read().len() == 0 {"Нажми, чтобы выбрать базу данных"} else {"Выбранная база данных: {input_file_path.read()}"}
                }
                input { 
                    id: "input_file",
                    class: "hidden",
                    r#type: "file",
                    oninput: move |event| async move {
                        if let Some(file_engine) = event.files() {
                            let files = file_engine.files();

                            if let Some(file_name) = files.get(0) {
                                input_file_path.set(file_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
}
