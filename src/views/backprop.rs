use std::error::Error;

use dioxus::{prelude::*};

use crate::{components::{BUTTON_CLASSES, BackpropMatrix, BackpropPrediction, Cluster, GoHome, INPUT_CLASSES, TabList}, utils::{ART1Clusters, ART1Config, BackpropElement, FileART1DatabaseReader, IART1DatabaseReader, INPUT_NEURONS_HEIGHT, INPUT_NEURONS_WIDTH, NeuralNetwork, OUTPUT_NEURONS, art1, load_dataset}};

#[component]
pub fn BackpropPage() -> Element {
    let mut input_file_path = use_signal(|| "".to_string());
    let mut epochs = use_signal(|| "".to_string());
    let mut learning_rate = use_signal(|| "".to_string());
    
    let mut matrix = use_signal(|| [[1.; INPUT_NEURONS_WIDTH]; INPUT_NEURONS_HEIGHT]);
    let mut model = use_signal(|| NeuralNetwork::new());

    rsx! {
        div {
            class: "flex flex-col gap-4",
            GoHome {  }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
            h3 {
                class: "text-gray-900 dark:text-white text-lg font-semibold",
                "База данных"
            }
            form {
                class: "contents",
                label { 
                    for: "input_file",
                    class: INPUT_CLASSES,
                    if input_file_path.read().len() == 0 {"Нажми, чтобы выбрать датасет"} else {"Выбранный датасет: {input_file_path.read()}"}
                }
                input {
                    directory: true,
                    id: "input_file",
                    class: "hidden",
                    r#type: "file",
                    oninput: move |event| async move {
                        if let Some(file_engine) = event.files() {
                            let files = file_engine.files();

                            if let Some(file_name) = files.get(0) {
                                input_file_path.set(file_name.to_string());
                            }
                        }
                    }
                }
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Количество эпох",
                type: "number",
                min: "1",
                value: "{epochs}",
                oninput: move |event| epochs.set(event.value())
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Скорость обучения",
                type: "number",
                min: "0.0",
                step: "0.001",
                value: "{learning_rate}",
                oninput: move |event| learning_rate.set(event.value())
            }
            button {
                class: BUTTON_CLASSES,
                onclick: move |_| {
                    let mut inner = move || -> Result<(), Box<dyn Error>> {
                        let param_input_file = input_file_path.read();

                        let dataset = load_dataset(param_input_file.as_str());

                        if dataset.len() == 0 {
                            return Err("Dataset is empty".into());
                        }
                        
                        let mut neural_network = NeuralNetwork::new();
                        let learning_rate = learning_rate.read().parse::<f64>()?;
                        for epoch in 0..epochs.read().parse::<usize>()? {
                            for data in dataset.iter() {
                                neural_network.set_inputs(&data);
                                neural_network.feed_forward();
                                neural_network.back_propagate(learning_rate);
                            }
                        }

                        model.set(neural_network);

                        Ok(())
                    };

                    match inner() {
                        Ok(_) => {},
                        Err(err) => {
                            println!("An error occured {err}");
                        }
                    }
                },
                "Обучить модель"
            }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
            BackpropMatrix {
                matrix: matrix.read().clone(),
                onclick: move |coords| {
                    // Обновить матрицу
                    let (x, y): (usize, usize) = coords;
                    let mut matrix_clone = matrix.read().clone();

                    matrix_clone[x][y] = if matrix_clone[x][y] > 0.5 { 0. } else { 1. };
                    matrix.set(matrix_clone);


                    // Выполнить предсказание
                    let mut model_clone = model.read().clone();
                    let model_element = BackpropElement {
                        data_in: matrix_clone.into_iter().flatten().collect::<Vec<f64>>().as_slice().try_into().unwrap(), 
                        out: [0.; OUTPUT_NEURONS]
                    };
                    model_clone.set_inputs(&model_element);
                    model_clone.feed_forward();
                    model.set(model_clone);
                }
            }
            BackpropPrediction {
                prediction: model.read().actual.clone()
            }
        }
    }
}
