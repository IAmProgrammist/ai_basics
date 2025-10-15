use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct InputLegendProps {
    title: String
}

#[component]
pub fn InputLegend(props: InputLegendProps) -> Element {
    rsx! {
        p {
            class: "text-gray-900 dark:text-white text-sm font-semibold",
            "{props.title}"
        }
    }
}
