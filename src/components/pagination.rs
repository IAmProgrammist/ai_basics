use dioxus::prelude::*;

use crate::components::BUTTON_CLASSES;

#[derive(Props, PartialEq, Clone)]
pub struct PaginationProps {
    page: usize,
    pages: usize,
    on_page_changes: Callback<usize>
}

#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let pages_clone = props.pages.clone();
    let on_page_changes_clone = props.on_page_changes.clone();

    rsx! {
        div {
            class: "flex flex-row p-1 items-center justify-between",
            div {
                class: BUTTON_CLASSES,
                onclick: move |_event| {
                    if props.pages == 0 {
                        return;
                    }

                    if props.page == 0 {
                        (props.on_page_changes)(props.pages - 1)    
                    }

                    (props.on_page_changes)((props.page - 1) % props.pages)
                },
                "<"
            },
            div {
                class: "text-lg text-gray-900 dark:text-white text-lg font-bold",
                "{props.page + 1}/{props.pages}"
            },
            div {
                class: BUTTON_CLASSES,
                onclick: move |_event| {
                    if props.pages == 0 {
                        return;
                    }

                    (on_page_changes_clone)((props.page + 1) % props.pages)
                },
                ">"
            }
        }
    }
}
