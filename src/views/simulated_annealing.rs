use dioxus::prelude::*;

use crate::components::{Field, GoHome, BUTTON_CLASSES, INPUT_CLASSES};
use crate::utils::{generate_field, simulated_annealing, FIELD_SIZE};
use dioxus_charts::LineChart;

#[component]
pub fn SimulatedAnnealingPage() -> Element {
    let mut field = use_signal(|| generate_field(0));
    let mut bad_decisions = use_signal(|| vec![0 as usize; 0]);
    let mut energy = use_signal(|| vec![0 as usize; 0]);
    let mut temperatures = use_signal(|| vec![0.0f64; 0]);

    let mut max_temp = use_signal(|| "30.0".to_string());
    let mut min_temp = use_signal(|| "0.5".to_string());
    let mut lower_coef = use_signal(|| "0.98".to_string());
    let mut queen_amount = use_signal(|| "20".to_string());
    let mut steps_amount = use_signal(|| "100".to_string());

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
                type: "number",
                min: "0.0",
                step: "0.001",
                value: "{max_temp}",
                oninput: move |event| max_temp.set(event.value())
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Минимальная температура",
                type: "number",
                min: "0.0",
                step: "0.001",
                value: "{min_temp}",
                oninput: move |event| min_temp.set(event.value())
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Коэффициент понижения температуры",
                type: "number",
                min: "0",
                max: "1",
                step: "0.001",
                value: "{lower_coef}",
                oninput: move |event| lower_coef.set(event.value())
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Количество ферзей",
                type: "number",
                min: 1,
                max: FIELD_SIZE * FIELD_SIZE,
                value: "{queen_amount}",
                oninput: move |event| queen_amount.set(event.value())
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Количество шагов при постоянном значении температуры",
                type: "number",
                min: 1,
                value: "{steps_amount}",
                oninput: move |event| steps_amount.set(event.value())
            }
            button {
                class: BUTTON_CLASSES,
                onclick: move |_| {
                    let max_temp = max_temp().parse::<f64>().unwrap();
                    let min_temp = min_temp().parse::<f64>().unwrap();
                    let lower_coef = lower_coef().parse::<f64>().unwrap();
                    let queen_amount = queen_amount().parse::<usize>().unwrap();
                    let steps_amount = steps_amount().parse::<usize>().unwrap();
                    
                    let result = simulated_annealing(max_temp, min_temp, lower_coef, queen_amount, steps_amount);
                    field.set(result.best_field);
                    bad_decisions.set(result.bad_decisions);
                    energy.set(result.energy);
                    temperatures.set(result.temperatures);
                },
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
                field: field()
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
                    label_interpolation: (|v| format!("{v}")) as fn(f32) -> String,
                    series: vec![bad_decisions().into_iter().map(|val| val as f32).collect()],
                    labels: bad_decisions().into_iter().enumerate().map(|(_, idx)| idx.to_string()).collect::<Vec<_>>(),
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
                    label_interpolation: (|v| format!("{v}")) as fn(f32) -> String,
                    series: vec![energy().into_iter().map(|val| val as f32).collect()],
                    labels: energy().into_iter().enumerate().map(|(_, idx)| idx.to_string()).collect::<Vec<_>>(),
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
                    label_interpolation: (|v| format!("{v}")) as fn(f32) -> String,
                    series: vec![temperatures().into_iter().map(|val| val as f32).collect()],
                    labels: temperatures().into_iter().enumerate().map(|(_, idx)| idx.to_string()).collect::<Vec<_>>(),
                }
                p {
                    class: "text-center font-semibold",
                    "Температура"
                }
            }
        }
    }
}
