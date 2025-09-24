use dioxus::prelude::*;

use crate::Route;

const LOGO_SMALL: Asset = asset!("/assets/logo_small.png");

#[component]
pub fn Logo() -> Element {
    rsx! {
        Link {
            to: Route::HomePage {},
            class: "flex flex-row items-center gap-2",
            img { 
                src: LOGO_SMALL,
                width: 64,
                height: 64,
            },
            h1 {
                class: "text-gray-900 dark:text-white text-2xl font-bold",
                "Основы ИИ"
            }
        }
    }
}
