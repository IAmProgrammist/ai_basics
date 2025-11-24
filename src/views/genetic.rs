use std::{error::Error, ops::Range, sync::Arc, thread::sleep, time::{Duration, Instant}};

use dioxus::{prelude::*};

use crate::{components::{BUTTON_CLASSES, GoHome, INPUT_CLASSES, InputLegend, MatrixDrawer}, utils::{ACOConfig, ACOPaths, Ant, MatrixACOPaths, aco, create_population, next_generation, parse_range}};

#[component]
pub fn GeneticPage() -> Element {
    let mut population_count = use_signal(|| "100".to_string());
    let mut mating_prob = use_signal(|| "0.5".to_string());
    let mut mutation_prob = use_signal(|| "0.1".to_string());

    let mut range_x_left = use_signal(|| "-10".to_string());
    let mut range_x_right = use_signal(|| "10".to_string());
    let mut range_y_bottom = use_signal(|| "-10".to_string());
    let mut range_y_top = use_signal(|| "10".to_string());
    let mut points_amount = use_signal(|| "10".to_string());
    let mut matrix: Signal<MatrixACOPaths> = use_signal(|| MatrixACOPaths::new(10, None, None));

    let mut simulation_threshold = use_signal(|| "4".to_string());
    let mut simulation_running = use_signal(|| false);

    let mut population = use_signal(|| create_population(10, 100));


    let mut force_reload = use_signal(|| 0);

    let mut best_len = use_signal(|| f64::INFINITY);
    let mut best_gene = use_signal(|| "".to_string());

    let mut best_current_len = use_signal(|| 0.);
    let mut best_current_gene = use_signal(|| "".to_string());

    /* 
    let mut best_ant = use_signal(|| Ant::new());
    let mut current_best_ant = use_signal(|| Ant::new());

    let best_ant_way = best_ant.read().visited.iter().fold(String::new(), |a, b| a + " " + &b.to_string());
    let current_best_ant_way = current_best_ant.read().visited.iter().fold(String::new(), |a, b| a + " " + &b.to_string());
*/
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
                title: "Размер популяции"
            }
            input { 
                class: INPUT_CLASSES,
                type: "number",
                min: "1",
                value: "{population_count}",
                oninput: move |event| population_count.set(event.value())
            }
            InputLegend {
                title: "Шанс скрещивания"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                min: "0.0",
                step: "0.01",
                value: "{mating_prob}",
                oninput: move |event| mating_prob.set(event.value())
            }
            InputLegend {
                title: "Шанс мутации"
            }
            input { 
                class: INPUT_CLASSES, 
                type: "number",
                min: "0.0",
                step: "0.01",
                value: "{mutation_prob}",
                oninput: move |event| mutation_prob.set(event.value())
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
                        let population_count = population_count.read().parse::<usize>()?;

                        matrix.set(MatrixACOPaths::new(points_amount, range_x, range_y));
                        force_reload.set(force_reload.clone() + 1);
                        /*
                        current_best_ant.set(Ant::new());
                        best_ant.set(Ant::new());
                        */
                        population.set(create_population(points_amount, population_count));

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
                        let points_amount = points_amount.read().parse::<usize>()?;
                        let population_count = population_count.read().parse::<usize>()?;

                        let mut matrix_copy = matrix.read().clone();
                        matrix_copy.clean_feromone();
                        matrix.set(matrix_copy);
                        force_reload.set(force_reload.clone() + 1);
                        population.set(create_population(points_amount, population_count));
                        Ok(())
                    };

                    match inner() {
                        Ok(_) => {},
                        Err(e) => println!("{}", e)
                    }
                },
                "Сбросить популяцию"
            }
            
            button {
                class: BUTTON_CLASSES,
                onclick: move |_| {
                    let mut threshold_arg: Option<usize> = None;
                    let mut mating_prob_arg: f64 = 1.;
                    let mut mutation_prob_arg: f64 = 1.;
                    let mut population_size_arg: usize = 100;
                    let mut should_run = false;

                    if !*simulation_running.read() {
                        let parser = move || -> Result<(f64, f64, usize, usize), Box<dyn Error>> { 
                            let mating_prob = mating_prob.read().parse::<f64>()?;
                            let mutation_prob = mutation_prob.read().parse::<f64>()?;
                            let population_size = population_count.read().parse::<usize>()?;

                            let threshold = simulation_threshold.read().parse::<usize>()?;

                            Ok((
                                mating_prob,
                                mutation_prob,
                                population_size,
                                threshold
                            ))
                        };

                        match parser() {
                            Ok((mating_prob, mutation_prob, population_size, threshold)) => {
                                threshold_arg = Some(threshold);
                                mating_prob_arg = mating_prob;
                                mutation_prob_arg = mutation_prob;
                                population_size_arg = population_size;
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

                        simulation_running.set(true);

                        let edit_matrix = matrix.read().clone();
                        let mut edit_matrix_arc: Arc<dyn ACOPaths> = Arc::new(edit_matrix);
                        
                        while *simulation_running.read() {
                            let species = population.read().clone();
                            let start_time = Instant::now();

                            let species_next = next_generation(&species, &mut edit_matrix_arc, mating_prob_arg, mutation_prob_arg, population_size_arg);

                            population.set(species_next.species);

                            best_current_len.set(species_next.best_len);
                            best_current_gene.set(species_next.best_genes.clone().iter().fold(String::new(), |a, b| a + " " + &b.to_string()));

                            if species_next.best_len < *best_len.read() {
                                best_len.set(species_next.best_len);
                                best_gene.set(species_next.best_genes.clone().iter().fold(String::new(), |a, b| a + " " + &b.to_string()));
                            }

                            let mut copy_paste_matrix = matrix.read().clone();
                            copy_paste_matrix = copy_paste_matrix.copy_fresh_and_feromone_from_trait(&mut edit_matrix_arc);
                            matrix.set(copy_paste_matrix);
                            force_reload.set(force_reload().clone() + 1);

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
            MatrixDrawer {
                matrix: Box::new(matrix.read().clone()),
                id: force_reload.read().clone(),
            }
            div {
                class: "text-gray-900 dark:text-white text-lg font-semibold",
                "Лучший ген"
            }
            div {
                class: "text-gray-900 dark:text-white text-base",
                "Длина пути: {best_len}"
            }
            div {
                class: "text-gray-900 dark:text-white text-base",
                "Путь: {best_gene}"
            }
            div {
                class: "text-gray-900 dark:text-white text-lg font-semibold",
                "Лучший текущий ген"
            }
            div {
                class: "text-gray-900 dark:text-white text-base",
                "Длина пути: {best_current_len}"
            }
            div {
                class: "text-gray-900 dark:text-white text-base",
                "Путь: {best_current_gene}"
            }
        }
    }
}
