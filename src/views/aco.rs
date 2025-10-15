use std::{error::Error, ops::Range, sync::Arc, thread::sleep, time::{Duration, Instant}};

use dioxus::{prelude::*};

use crate::{components::{GoHome, InputLegend, BUTTON_CLASSES, INPUT_CLASSES}, utils::{aco, parse_range, ACOConfig, ACOPaths, MatrixACOPaths}};

#[component]
pub fn ACOPage() -> Element {
    let mut ants = use_signal(|| "3".to_string());
    let mut feromone_weight = use_signal(|| "1".to_string());
    let mut heuristic_coefficient = use_signal(|| "3.5".to_string());
    let mut evaporation_coefficient = use_signal(|| "0.2".to_string());
    let mut q = use_signal(|| "10".to_string());
    let mut begin_city = use_signal(|| "0".to_string());
    let mut target_city = use_signal(|| "1".to_string());

    let mut range_x_left = use_signal(|| "-10".to_string());
    let mut range_x_right = use_signal(|| "10".to_string());
    let mut range_y_bottom = use_signal(|| "-10".to_string());
    let mut range_y_top = use_signal(|| "10".to_string());
    let mut points_amount = use_signal(|| "5".to_string());
    let mut matrix: Signal<MatrixACOPaths> = use_signal(|| MatrixACOPaths::new(5, None, None));

    let mut simulation_threshold = use_signal(|| "500".to_string());
    let mut simulation_running = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-col gap-4",
            GoHome {  }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
            h3 {
                class: "text-gray-900 dark:text-white text-lg font-semibold",
                "Симуляция"
            }
            InputLegend {
                title: "Количество муравьёв"
            }
            input { 
                class: INPUT_CLASSES,
                type: "number",
                min: "1",
                value: "{ants}",
                oninput: move |event| ants.set(event.value())
            }
            InputLegend {
                title: "Коэффициент значимости феромона"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                min: "0.0",
                step: "0.01",
                value: "{feromone_weight}",
                oninput: move |event| feromone_weight.set(event.value())
            }
            InputLegend {
                title: "Коэффициент эвристики"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                min: "0.0",
                step: "0.01",
                value: "{heuristic_coefficient}",
                oninput: move |event| heuristic_coefficient.set(event.value())
            }
            InputLegend {
                title: "Коэффициент испарения"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                min: "0.0",
                step: "0.01",
                value: "{evaporation_coefficient}",
                oninput: move |event| evaporation_coefficient.set(event.value())
            }
            InputLegend {
                title: "Q"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                min: "0.0",
                step: "0.01",
                value: "{q}",
                oninput: move |event| q.set(event.value())
            }
            InputLegend {
                title: "Индекс города-муравейника"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                min: "0",
                max: points_amount.clone(),
                value: "{begin_city}",
                oninput: move |event| begin_city.set(event.value())
            }
            InputLegend {
                title: "Индекс города-назначения"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                min: "0",
                max: points_amount.clone(),
                value: "{target_city}",
                oninput: move |event| target_city.set(event.value())
            }
            InputLegend {
                title: "Скорость симуляции (время между итерациями в мс.)"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                min: "0",
                value: "{simulation_threshold}",
                oninput: move |event| simulation_threshold.set(event.value())
            }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
            h3 {
                class: "text-gray-900 dark:text-white text-lg font-semibold",
                "Карта"
            }
            InputLegend {
                title: "Минимальный x"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                step: "0.01",
                value: "{range_x_left}",
                oninput: move |event| range_x_left.set(event.value())
            }
            InputLegend {
                title: "Максимальный x"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                step: "0.01",
                value: "{range_x_right}",
                oninput: move |event| range_x_right.set(event.value())
            }
            InputLegend {
                title: "Минимальный y"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                step: "0.01",
                value: "{range_y_bottom}",
                oninput: move |event| range_y_bottom.set(event.value())
            }
            InputLegend {
                title: "Максимальный y"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                step: "0.01",
                value: "{range_y_top}",
                oninput: move |event| range_y_top.set(event.value())
            }
            InputLegend {
                title: "Количество городов"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                min: "0",
                value: "{points_amount}",
                oninput: move |event| points_amount.set(event.value())
            }
            button {
                class: BUTTON_CLASSES,
                onclick: move |_| {
                    if *simulation_running.read() {
                        return;
                    }

                    let mut inner = move || -> Result<(), Box<dyn Error>> {
                        let range_x = parse_range(range_x_left.read().to_string(), range_x_right.read().to_string());
                        let range_y = parse_range(range_y_bottom.read().to_string(), range_y_top.read().to_string());
                        let points_amount = points_amount.read().parse::<usize>()?;

                        matrix.set(MatrixACOPaths::new(points_amount, range_x, range_y));

                        Ok(())
                    };

                    match inner() {
                        Ok(_) => {},
                        Err(e) => println!("{}", e)
                    }
                },
                "Сгенерировать карту"
            }
            button {
                class: BUTTON_CLASSES,
                onclick: move |_| {
                    if *simulation_running.read() {
                        return;
                    }

                    let mut inner = move || -> Result<(), Box<dyn Error>> {
                        let mut matrix_copy = matrix.read().clone();
                        matrix_copy.clean_feromone();
                        matrix.set(matrix_copy);
                        Ok(())
                    };

                    match inner() {
                        Ok(_) => {},
                        Err(e) => println!("{}", e)
                    }
                },
                "Очистить феромон"
            }
            
            button {
                class: BUTTON_CLASSES,
                onclick: move |_| {
                    let mut threshold_arg: Option<usize> = None;
                    let mut config_arg: Option<ACOConfig> = None;
                    let mut should_run = false;

                    if !*simulation_running.read() {
                        let mut parser = move || -> Result<(ACOConfig, usize), Box<dyn Error>> { 
                            let ants = ants.read().parse::<usize>()?;
                            let feromone_weight = feromone_weight.read().parse::<f64>()?;
                            let heuristic_coefficient = heuristic_coefficient.read().parse::<f64>()?;
                            let evaporation_coefficient = evaporation_coefficient.read().parse::<f64>()?;
                            let q = q.read().parse::<f64>()?;
                            let begin_city = begin_city.read().parse::<usize>()?;
                            let target_city = target_city.read().parse::<usize>()?;

                            let threshold = simulation_threshold.read().parse::<usize>()?;

                            Ok((
                                ACOConfig {ants, feromone_weight, heuristic_coefficient, q, begin_city: Some(begin_city), target_city, evaporation_coefficient},
                                threshold
                            ))
                        };

                        match parser() {
                            Ok((config, threshold)) => {
                                threshold_arg = Some(threshold);
                                config_arg = Some(config);
                                should_run = true;
                            },
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                    } else {
                        simulation_running.set(false);
                    }

                    return async move {
                        if !should_run {
                            return;
                        }

                        let threshold = threshold_arg.unwrap();
                        let config = config_arg.unwrap();

                        simulation_running.set(true);

                        let edit_matrix = matrix.read().clone();
                        let mut edit_matrix_arc: Arc<dyn ACOPaths> = Arc::new(edit_matrix);
                        
                        while *simulation_running.read() {
                            let start_time = Instant::now();
                            let _ = aco(&config, &mut edit_matrix_arc).await;

                            for i in 0..matrix.read().len() {
                                for j in 0..matrix.read().len() {
                                    print!("{:.2} ", matrix.read().get_feromone_intensity(i, j).unwrap_or(0.));
                                }
                                println!("");
                            }
                            println!("");
                            println!("");

                            let mut copy_paste_matrix = matrix.read().clone();
                            copy_paste_matrix = copy_paste_matrix.copy_fresh_and_feromone_from_trait(&mut edit_matrix_arc);
                            matrix.set(copy_paste_matrix);

                            let ellapsed = start_time.elapsed();
                            let ellapsed = ellapsed.as_millis();
                            if ellapsed > threshold as u128 {
                                continue;
                            }
                            let wait_time = threshold as u128 - ellapsed;
                            tokio::time::sleep(Duration::from_millis(wait_time as u64)).await;
                        }
                    };
                },
                if *simulation_running.read() { "Приостановить симуляцию" } else { "Запустить симуляцию" }
            }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
        }
    }
}
