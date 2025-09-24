use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn GoHome() -> Element {
    rsx! {
        Link {
            class: 
            "hover:bg-gray-400 dark:hover:bg-gray-500
             active:bg-gray-500 dark:active:bg-gray-400
             flex flex-row rounded-md p-1 text-lg text-gray-900 dark:text-white
             items-center gap-2",
            to: Route::HomePage {},
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                height: 24,
                width: 24,
                view_box: "0 0 24 24",
                fill: "currentColor",
                g {
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_width: "1.5",
                    path {
                        stroke_linejoin: "round",
                        d: "m13.5 9l-3 3l3 3"
                    },
                    path {
                        d: "M7 3.338A9.954 9.954 0 0 1 12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12c0-1.821.487-3.53 1.338-5"
                    }
                }
            }
            "К списку лабораторных работ"
        }
    }
}
