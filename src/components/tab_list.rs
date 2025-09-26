use dioxus::prelude::*;

use crate::components::BUTTON_CLASSES;

#[derive(Props, PartialEq, Clone)]
pub struct TabListProps {
    page: usize,
    pages: Vec<String>,
    on_page_changes: impl Fn(usize) -> ()
}

#[component]
pub fn TabList(props: TabListProps) -> Element {
    let pages_clone = props.pages.clone();

    rsx! {
        div {
            class: "flex flex-row p-1 items-center justify-between",
            div {
                class: BUTTON_CLASSES,
                onclick: move |_event| {
                    (props.on_page_changes)((props.pages.len() - 1) % props.pages.len())
                },
                "Пред."
            },
            div {
                class: "text-lg text-gray-900 dark:text-white text-lg font-bold",
                "{props.pages[props.page % props.pages.len()]}"
            },
            div {
                class: BUTTON_CLASSES,
                onclick: move |_event| {
                    (props.on_page_changes)((pages_clone.len() + 1) % pages_clone.len())
                },
                "След."
            }
        }
    }
}
