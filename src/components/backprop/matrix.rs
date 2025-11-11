use dioxus::{html::u::background_color, prelude::*};

use crate::{utils::{INPUT_NEURONS_HEIGHT, INPUT_NEURONS_WIDTH}};

#[derive(Props, PartialEq, Clone)]
pub struct BackpropMatrixProps {
    matrix: [[f64; INPUT_NEURONS_WIDTH]; INPUT_NEURONS_HEIGHT],
    onclick: Callback<(usize, usize)>
}

#[component]
pub fn BackpropMatrix(props: BackpropMatrixProps) -> Element {
    rsx! {
        div {
            class: "grid grid-cols-5 grid-rows-7 gap-1 aspect-5/7",
            for i in 0..INPUT_NEURONS_HEIGHT {
                for j in 0..INPUT_NEURONS_WIDTH {
                    div {
                        class: "rounded-md border-solid border border-gray-300 min-h-6",
                        style: "background-color: rgb({props.matrix[i][j] * 256.}, {props.matrix[i][j] * 256.}, {props.matrix[i][j] * 256.})",
                        onclick: move |_| {
                            (props.onclick)((i, j))
                        },
                        ""
                    }
                }
            }
        }
    }
}