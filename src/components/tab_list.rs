use dioxus::prelude::*;

use crate::components::BUTTON_CLASSES;

#[derive(Props, PartialEq, Clone)]
pub struct TabListProps {
    page: usize,
    pages: Vec<String>,
    on_page_changes: Callback<usize>
}

#[component]
pub fn TabList(props: TabListProps) -> Element {
    let pages_clone = props.pages.clone();
    let on_page_changes_clone = props.on_page_changes.clone();

    rsx! {
        div {
            class: "flex flex-row p-1 items-center justify-between",
            div {
                class: BUTTON_CLASSES,
                onclick: move |_event| {
                    if props.pages.len() == 0 {
                        return;
                    }

                    if props.page == 0 {
                        (props.on_page_changes)(props.pages.len() - 1);
                        return;
                    }

                    (props.on_page_changes)((props.page - 1) % props.pages.len())
                },
                "Пред."
            },
            if props.pages.len() == 0 { 
                div {
                    class: "text-lg text-gray-900 dark:text-white text-lg font-bold",
                    "Страниц нет :("
                }
            } else 
            {
                div {
                    class: "text-lg text-gray-900 dark:text-white text-lg font-bold",
                    "{props.pages[props.page % props.pages.len()]}"
                }
            },
            div {
                class: BUTTON_CLASSES,
                onclick: move |_event| {
                    if pages_clone.len() == 0 {
                        return;
                    }

                    (on_page_changes_clone)((props.page + 1) % pages_clone.len())
                },
                "След."
            }
        }
    }
}
