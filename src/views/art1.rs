use std::{error::Error, thread::spawn};

use dioxus::{prelude::*};

use crate::{components::{Cluster, GoHome, TabList, BUTTON_CLASSES, INPUT_CLASSES}, utils::{art1, ART1Clusters, ART1Config, FileART1DatabaseReader, IART1DatabaseReader}};

#[component]
pub fn ART1Page() -> Element {
    let mut input_file_path = use_signal(|| "".to_string());
    let mut max_clusters = use_signal(|| "".to_string());
    let mut beta_coef = use_signal(|| "".to_string());
    let mut attention_coef = use_signal(|| "".to_string());
    let mut clusters_page = use_signal(|| 0 as usize);

    let mut clusters = use_signal(|| ART1Clusters {clusters: vec![]});
    let mut cluster_dimension = use_signal(|| 0);

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
                    if input_file_path.read().len() == 0 {"Нажми, чтобы выбрать базу данных"} else {"Выбранная база данных: {input_file_path.read()}"}
                }
                input { 
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
                placeholder: "Максимальное количество кластеров",
                type: "number",
                min: "1",
                value: "{max_clusters}",
                oninput: move |event| max_clusters.set(event.value())
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Бета-параметр",
                type: "number",
                min: "0.0",
                step: "0.001",
                value: "{beta_coef}",
                oninput: move |event| beta_coef.set(event.value())
            }
            input { 
                class: INPUT_CLASSES, 
                placeholder: "Параметр внимательности",
                type: "number",
                min: "0",
                max: "1",
                step: "0.001",
                value: "{attention_coef}",
                oninput: move |event| attention_coef.set(event.value())
            }
            button {
                class: BUTTON_CLASSES,
                onclick: move |_| {
                    let mut inner = move || -> Result<(), Box<dyn Error>> {
                        let param_input_file = input_file_path();

                        let db = FileART1DatabaseReader::new(
                            &param_input_file
                        ).read()?;

                        let config = ART1Config {
                            max_clusters: max_clusters().parse::<usize>()?,
                            beta: beta_coef().parse::<f64>()?,
                            attention: attention_coef().parse::<f64>()?
                        };

                        let out = art1(&db, &config)?;
                      
                        clusters.set(out);
                        cluster_dimension.set(db.dimension);
                        clusters_page.set(0);

                        Ok(())
                    };

                    match inner() {
                        Ok(_) => {},
                        Err(e) => {
                            println!("{}", e)
                        }
                    }
                },
                "Запустить кластеризацию"
            }
            hr { 
                class: "text-gray-900 dark:text-white"
            }
            TabList {
                page: clusters_page(),
                pages: clusters.read().clusters.iter().enumerate().map(|(number, _it)| {
                    let number = number + 1;
                    format!("Кластер {number}")
                }).collect(),
                on_page_changes: move |new_page: usize| {
                    clusters_page.set(new_page);
                }
            }
            Cluster {
                dimension: *cluster_dimension.read(),
                cluster: if clusters_page() < clusters.read().clusters.len() {clusters.read().clusters[clusters_page()].clone()} else {vec![]}
            }
        }
    }
}
