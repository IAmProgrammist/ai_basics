use std::{error::Error, sync::{mpsc::channel, Arc, RwLock, RwLockReadGuard}, vec};

use rand::Rng;
use threadpool::ThreadPool;

use crate::utils::{ACOConfig, ACOPaths, Ant};

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::utils::{aco, ACOConfig, ACOPaths, MatrixACOPaths};

    // Test functions and helper code go here
    #[test]
    fn it_works() {
        let mut paths: Arc<dyn ACOPaths> = Arc::new(MatrixACOPaths::new(5));
        let aco_config = ACOConfig { ants: 2, feromone_weight: 0.5, heuristic_coefficient: 0.5, q: 10., target_city: 2, evaporation_coefficient: 0.1 };
        for i in 0..5 {
            for j in 0..5 {
                print!("{:.4} ", paths.get_feromone_intensity(i, j).unwrap())
            }
            println!("");
        }
        for _ in 0..100 {
            paths = aco(&aco_config, paths).unwrap();
            println!("GAY");
            for i in 0..5 {
                for j in 0..5 {
                    print!("{:.4} ", paths.get_feromone_intensity(i, j).unwrap())
                }
                println!("");
            }
        }
    }
}

pub fn aco(config: &ACOConfig, mut paths: Arc<dyn ACOPaths>) -> Result<Arc<dyn ACOPaths>, Box<dyn Error>> {
    let (tx, rx) = channel::<Ant>();
    {
        let field = Arc::new(RwLock::new(paths.clone()));
        
        let cpu_count = num_cpus::get_physical();
        let pool = ThreadPool::new(cpu_count);
        let ants_amount = config.ants;
        
        for _worker in 0..ants_amount {
            let tx = tx.clone();
            let field_clone = Arc::clone(&field);
            let config = config.clone();

            pool.execute(move || {
                let mut rng = rand::rng();
                let field_read = field_clone.read();
                let mut ant = Ant::new();
                if field_read.is_err() {
                    let _ = tx.send(ant);
                    return;
                }

                // Если инициализация, ставим в случайную точку. Иначе - в ноль.
                let field_read = field_read.unwrap();
                if field_read.is_fresh() {
                    ant.visited.push(rng.random_range(0..config.ants));
                } else {
                    ant.visited.push(0);
                }
                loop {
                    let field_read = field_clone.read().unwrap();
                    match find_next_city(&config, field_read, &ant) {
                        Ok(city_index) => {
                            ant.visited.push(city_index);
                            if city_index == config.target_city {
                                let _ = tx.send(ant);
                                return;
                            }
                        },
                        Err(e) => {
                            println!("Whoopsie for ant: {}", e);
                            let _ = tx.send(Ant::new());
                            return;
                        }
                    }
                }
            });
        }

        pool.join();
    }
    {
        for ant in rx.iter().take(config.ants) {
            let paths = Arc::get_mut(&mut paths).ok_or("Failed")?;

            if ant.visited.len() == 0 {
                continue;
            }

            let feromone_added = get_ant_path_feromone(config, paths, &ant);

            for path_index in 0..(ant.visited.len() - 1) {
                paths.set_feromone_intensity(feromone_added + paths.get_distance(ant.visited[path_index], ant.visited[path_index + 1]).unwrap_or(0.) * config.evaporation_coefficient, 
                ant.visited[path_index], 
                ant.visited[path_index + 1])?;
            }
        }

        let paths = Arc::get_mut(&mut paths).ok_or("Failed")?;
        evaporate(config, paths)?;
    }

    Ok(paths)
}

fn find_next_city(config: &ACOConfig, paths: RwLockReadGuard<'_, Arc<dyn ACOPaths>>, ant: &Ant) -> Result<usize, Box<dyn Error>> {
    let mut probs = vec![0.; 0];
    let mut indices = vec![0; 0];
    let mut denom: f64 = 0.;
    let ant_current_index = ant.visited.last().ok_or("Ant should have at least one point")?;
    let mut rng = rand::rng();

    for city_index in 0..paths.len() {
        if let Some(_) = ant.visited.iter().find(|visited_city_index| **visited_city_index == city_index) {
            continue;
        }

        indices.push(city_index);
        let product = 
            paths.get_feromone_intensity(*ant_current_index, city_index)?.powf(config.feromone_weight) * 
            (1. / paths.get_distance(*ant_current_index, city_index)? as f64)
            .powf(config.heuristic_coefficient);
        denom += product;
        probs.push(product);
    }

    if denom == 0.0 {
        return Err("Luck is not on your side, cowboy".into());
    }

    probs = probs.iter().map(|pr| pr / denom).collect();
    let probability_sum = probs.iter().copied().reduce(|acc, e| acc + e).unwrap_or(0.);
    let curr_prob = rng.random_range(0.0..probability_sum);

    let mut left_prob: f64 = 0.;
    for allowed_city_index in 0..probs.len() {
        if left_prob <= curr_prob && curr_prob <= left_prob + probs[allowed_city_index] {
            return Ok(indices[allowed_city_index])
        }

        left_prob += probs[allowed_city_index];
    }

    Ok(indices[0])
}

fn get_ant_path_feromone(config: &ACOConfig, paths: &mut dyn ACOPaths, ant: &Ant) -> f64 {
    config.q / get_ant_path_length(paths, ant)
}

fn get_ant_path_length(paths: &mut dyn ACOPaths, ant: &Ant) -> f64 {
    let mut path_len = 0.0;
    
    for path_index in 0..(ant.visited.len() - 1) {
        path_len += paths.get_distance(ant.visited[path_index], ant.visited[path_index + 1]).unwrap_or(0.);
    }

    path_len
}

fn evaporate(config: &ACOConfig, paths: &mut dyn ACOPaths) -> Result<(), Box<dyn Error>> {
    for i in 0..paths.len() {
        for j in 0..paths.len() {
            paths.set_feromone_intensity(paths.get_feromone_intensity(i, j)? * (1. - config.evaporation_coefficient), i, j)?;
        }
    }

    Ok(())
}