use dioxus::{html::u::background_color, prelude::*};

use crate::utils::{BACKPROPS_OPERATIONS_LOCALIZATIONS, OUTPUT_NEURONS};

#[derive(Props, PartialEq, Clone)]
pub struct BackpropPredictionProps {
    prediction: [f64; OUTPUT_NEURONS]
}

#[component]
pub fn BackpropPrediction(props: BackpropPredictionProps) -> Element {
    let mut ratings: Vec<(f64, usize)> = props.prediction
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, index))
        .collect();

    ratings.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    rsx! {
        div {
            class: "rounded-md border-solid border border-gray-300 flex flex-col gap-3 p-4",
            for rating in ratings.iter() {
                p {
                    class: "text-gray-900 dark:text-white text-lg font-bold",
                    "{BACKPROPS_OPERATIONS_LOCALIZATIONS[rating.1]}: {rating.0}"
                }
            } 
        }
    }
}