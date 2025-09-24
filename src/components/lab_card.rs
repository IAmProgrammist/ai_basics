use dioxus::prelude::*;

use crate::Route;

#[derive(Props, PartialEq, Clone)]
pub struct LabCardProps {
    number: u32,
    sub_title: String,
    redirect: Route
}

#[component]
pub fn LabCard(props: LabCardProps) -> Element {
    rsx! {
        Link {
            class: 
            "rounded-md border-solid border
            bg-gray-300 dark:bg-gray-600 dark:border-gray-300  border-gray-600 
            hover:bg-gray-400 dark:hover:bg-gray-500
            active:bg-gray-500 dark:active:bg-gray-400
            p-3 flex flex-col",
            to: props.redirect,
            h2 {
                class: "text-gray-900 dark:text-white text-xl font-medium",
                "Лабораторная работа №{props.number}"
            },
            p {
                class: "text-gray-900 dark:text-white text-base font-medium",
                "Тема: {props.sub_title}"
            },
        }
    }
}
