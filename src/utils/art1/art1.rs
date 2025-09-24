use std::error::Error;

use crate::utils::{ART1Clusters, ART1Database};

pub struct ART1Config {
    pub max_clusters: usize,
    pub attention: f64,
    pub beta: f64,
}

pub fn art1(database: &ART1Database, config: &ART1Config) -> Result<ART1Clusters, Box<dyn Error>> {
    let mut art1_clusters = ART1Clusters { clusters: vec![] };

    if database.dimension == 0 || database.dataset.len() == 0 {
        return Ok(art1_clusters);
    }

    // Создаём первый кластер с единственным вектором-прототипом -- первым элементом из списка.
    create_cluster(config, &mut art1_clusters, database.dataset[0])?;

    // Выполнить кластеризацию
    for database_element in database.dataset.iter() {
        // Найти подходящий кластер
        let op_cluster_index = find_fitting_cluster(database, config, &art1_clusters, *database_element)?;
        match op_cluster_index {
            Some(cluster_index) => {
                // Если кластер найден, то добавляем в него элемент
                push_cluster(&mut art1_clusters, cluster_index, *database_element)?;
            },
            None => {
                // Иначе, пытаемся создать новый кластер
                match create_cluster(&config, &mut art1_clusters, *database_element) {
                    Err(error) => eprintln!("{error}"),
                    Ok(_) => {}
                }
            }
        }
    }

    Ok(art1_clusters)
}

fn create_cluster(
    config: &ART1Config,
    clusters: &mut ART1Clusters,
    prototype: u64
) -> Result<(), Box<dyn Error>> {
    if clusters.clusters.len() >= config.max_clusters {
        return Err(format!("Clusters are overflowing. Consider expanding").into());
    }

    clusters
        .clusters
        .push(vec![prototype]);

    Ok(())
}

fn push_cluster(
    clusters: &mut ART1Clusters,
    cluster_index: usize,
    value: u64
) -> Result<(), Box<dyn Error>> {
    if cluster_index >= clusters.clusters.len() {
        return Err(format!("A cluster with index {cluster_index} doesn't exists").into());
    }

    clusters.clusters[cluster_index].push(value);
    Ok(())
}

// Найти подходящий кластер
fn find_fitting_cluster(
    database: &ART1Database,
    config: &ART1Config,
    clusters: &ART1Clusters,
    value: u64
) -> Result<Option<usize>, Box<dyn Error>> {
    for cluster_index in 0..clusters.clusters.len() {
        if clusters.clusters[cluster_index].len() == 0 {
            return Err("Prototype is missing in cluster".into());
        }

        let prototype = clusters.clusters[cluster_index][0];

        // Проводим проверку на схожесть и на внимание
        if check_similarity(database, config, prototype, value) && 
        check_attention(config, prototype, value) {
            return Ok(Some(cluster_index));
        }
    }

    Ok(None)
}

// Проверка на схожесть
fn check_similarity(
    database: &ART1Database,
    config: &ART1Config,
    prototype: u64,
    value: u64,
) -> bool {
    ((prototype & value).count_ones() as f64 / (config.beta + prototype.count_ones() as f64))
        > (value.count_ones() as f64 / (config.beta + database.dimension as f64))
}

// Проверка на внимание
fn check_attention(
    config: &ART1Config,
    prototype: u64,
    value: u64,
) -> bool {
    ((prototype & value).count_ones() as f64 / value.count_ones() as f64) < config.attention
}