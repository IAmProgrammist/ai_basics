use dioxus::prelude::*;

use crate::components::{Field, GoHome, BUTTON_CLASSES, INPUT_CLASSES};
use dioxus_charts::LineChart;

const MATRIX_SIZE: usize = 20;

#[component]
pub fn SimulatedAnnealingPage() -> Element {
    let field: Vec<Vec<bool>> = vec![vec![true; MATRIX_SIZE]; MATRIX_SIZE];

    rsx! {
        div {
            class: "flex flex-col gap-4",
            GoHome {  }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
            h3 {
                class: "text-gray-900 dark:text-white text-lg font-semibold",
                "Входные значения"
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Максимальная температура",
                type: "number"
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Минимальная температура",
                type: "number"
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Коэффициент понижения температуры",
                type: "number"
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Количество ферзей",
                type: "number"
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Количество шагов при постоянном значении температуры",
                type: "number"
            }
            button {
                class: BUTTON_CLASSES,
                "Запустить симуляцию"
            }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
            h3 {
                class: "text-gray-900 dark:text-white text-lg font-semibold",
                "Шахматная доска"
            }
            Field {
                field: field
            }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
            h3 {
                class: "text-gray-900 dark:text-white text-lg font-semibold",
                "Графики"
            }
            div {
                class: "bg-gray-100 border border-solid border-grey-800 rounded p-4",
                LineChart {
                    width: "100%",
                    height: "100%",
                    padding_top: 30,
                    padding_left: 50,
                    padding_right: 90,
                    padding_bottom: 30,
                    show_grid_ticks: true,
                    show_dotted_grid: false,
                    label_interpolation: (|v| format!("{v}%")) as fn(f32) -> String,
                    series: vec![
                        vec![75.77, 73.95, 74.56, 78.25, 77.15, 62.64, 67.51],
                        vec![57.17, 57.78, 54.69, 52.95, 51.78, 41.0, 47.25],
                        vec![23.12, 26.5, 26.1, 29.84, 25.05, 20.41, 20.1],
                        vec![26.02, 21.48, 21.05, 22.64, 20.64, 17.19, 16.31],
                        vec![0.0, 13.65, 12.3, 13.35, 11.17, 9.87, 10.15],
                    ],
                    labels: vec!["2016".into(), "2017".into(), "2018".into(), "2019".into(), "2020".into(), "2021".into(), "2022".into()],
                    series_labels: vec!["Firefox".into(), "Chromium".into(), "Chrome".into(), "Epiphany".into(), "Konqueror".into()],
                }
                p {
                    class: "text-center font-semibold",
                    "Плохие решения"
                }
            }
            div {
                class: "bg-gray-100 border border-solid border-grey-800 rounded p-4",
                LineChart {
                    width: "100%",
                    height: "100%",
                    padding_top: 30,
                    padding_left: 50,
                    padding_right: 90,
                    padding_bottom: 30,
                    show_grid_ticks: true,
                    show_dotted_grid: false,
                    label_interpolation: (|v| format!("{v}%")) as fn(f32) -> String,
                    series: vec![
                        vec![75.77, 73.95, 74.56, 78.25, 77.15, 62.64, 67.51],
                        vec![57.17, 57.78, 54.69, 52.95, 51.78, 41.0, 47.25],
                        vec![23.12, 26.5, 26.1, 29.84, 25.05, 20.41, 20.1],
                        vec![26.02, 21.48, 21.05, 22.64, 20.64, 17.19, 16.31],
                        vec![0.0, 13.65, 12.3, 13.35, 11.17, 9.87, 10.15],
                    ],
                    labels: vec!["2016".into(), "2017".into(), "2018".into(), "2019".into(), "2020".into(), "2021".into(), "2022".into()],
                    series_labels: vec!["Firefox".into(), "Chromium".into(), "Chrome".into(), "Epiphany".into(), "Konqueror".into()],
                }
                p {
                    class: "text-center font-semibold",
                    "Энергия лучшего решения"
                }
            }
            div {
                class: "bg-gray-100 border border-solid border-grey-800 rounded p-4",
                LineChart {
                    width: "100%",
                    height: "100%",
                    padding_top: 30,
                    padding_left: 50,
                    padding_right: 90,
                    padding_bottom: 30,
                    show_grid_ticks: true,
                    show_dotted_grid: false,
                    label_interpolation: (|v| format!("{v}%")) as fn(f32) -> String,
                    series: vec![
                        vec![75.77, 73.95, 74.56, 78.25, 77.15, 62.64, 67.51],
                        vec![57.17, 57.78, 54.69, 52.95, 51.78, 41.0, 47.25],
                        vec![23.12, 26.5, 26.1, 29.84, 25.05, 20.41, 20.1],
                        vec![26.02, 21.48, 21.05, 22.64, 20.64, 17.19, 16.31],
                        vec![0.0, 13.65, 12.3, 13.35, 11.17, 9.87, 10.15],
                    ],
                    labels: vec!["2016".into(), "2017".into(), "2018".into(), "2019".into(), "2020".into(), "2021".into(), "2022".into()],
                    series_labels: vec!["Firefox".into(), "Chromium".into(), "Chrome".into(), "Epiphany".into(), "Konqueror".into()],
                }
                p {
                    class: "text-center font-semibold",
                    "Температура"
                }
            }
        }
    }
}
