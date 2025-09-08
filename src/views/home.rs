use crate::{components::LabCard, Route};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn HomePage() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2",
            p {
                class: "text-gray-900 dark:text-white",
                "Список лабораторных работ:"
            }
            div {
                class: "flex flex-col gap-2",
                LabCard {
                    number: 1,
                    sub_title: "Алгоритм отжига",
                    redirect: Route::SimulatedAnnealingPage {  }
                }
            }
        }
    }
}
