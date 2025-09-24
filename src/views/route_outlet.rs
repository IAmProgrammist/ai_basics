use crate::{components::Logo, Route};
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn RouteOutlet() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        header {
            class: "flex flex-row py-4",
            Logo {  }
        }

        main {
            class: "dark:bg-gray-700 dark:border-gray-200 bg-gray-200 border-gray-700 p-3 rounded-md border-solid border",
            Outlet::<Route> {}
        }

        footer {
            class: "text-center text-gray-900 dark:text-white py-4",
            "IAmProgrammist, 2025"
        }
    }
}
