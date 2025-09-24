use dioxus::prelude::*;

use crate::components::{GoHome};

#[component]
pub fn ART1Page() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-4",
            GoHome {  }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
        }
    }
}
