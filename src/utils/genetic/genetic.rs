use std::{error::Error, io::Read, sync::{Arc, RwLock, RwLockReadGuard}};

use rand::Rng;

use crate::utils::ACOPaths;

// Функция-обёртка алгоритма скрещивания, позволяющая выбрать случайные индексы смешения генов
fn cross_parent(parent_a: &Vec<usize>, parent_b: &Vec<usize>) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut rng = rand::rng();
    let gene_index_start = rng.random_range((0..parent_a.len()));
    let gene_index_end = rng.random_range((gene_index_start..parent_a.len()));

    return cross_within_mixup_range(parent_a, parent_b, gene_index_start, gene_index_end);
}

// Смешивает гены родителей и возвращает ребёнка
fn cross_within_mixup_range(parent_a: &Vec<usize>, parent_b: &Vec<usize>, mixup_low: usize, mixup_higher: usize) -> Result<Vec<usize>, Box<dyn Error>> {
    assert!(mixup_higher >= mixup_low);
    assert!(parent_a.len() == parent_b.len());

    let mut child = parent_b.clone();

    let mut parent_a_gene_index: usize = 0;
    let mut child_gene_index: usize = 0;

    while parent_a_gene_index < parent_a.len() && child_gene_index < child.len() {
        cross_find_next_gene_index(parent_a, mixup_low, mixup_higher, &mut parent_a_gene_index);
        cross_find_next_gene_index(&child, mixup_low, mixup_higher, &mut child_gene_index);

        if parent_a_gene_index >= parent_a.len() || child_gene_index >= child.len() {
            break;
        }

        child[child_gene_index] = parent_a[parent_a_gene_index];
        parent_a_gene_index += 1;
        child_gene_index += 1;
    }

    Ok(child)
}

fn cross_find_next_gene_index(specie_gens: &Vec<usize>, mixup_low: usize, mixup_higher: usize, gene_index: &mut usize) {
    while *gene_index < specie_gens.len() && specie_gens[*gene_index] < mixup_low || specie_gens[*gene_index] > mixup_higher {
        *gene_index += 1;
    }
}

fn cross(species: Vec<Vec<usize>>, mate_prob: f64) -> Vec<Vec<usize>> {
    // Выполняет скрещивание
    let mut cross_result = vec![vec![0 as usize; species[0].len()]; species.len()];
    let mut cross_result_idx = 0 as usize;

    // Выполним построение массива встреч
    let mut rng = rand::rng();
    let mut mating_arr: Vec<usize> = (0..(species.len() * 2)).map(|_| {
        rng.random_range(0..species.len())
    }).collect();

    for mate_idx in (0..mating_arr.len()).step_by(2) {
        // Если шанс встречи подходит, то выполняем скрещивание
        let mate_chance = rng.random_range((0.)..(1.));
        if mate_chance > mate_prob || mate_idx + 1 >= mating_arr.len() {
            continue;
        }

        let parent_a: Vec<usize> = species[mating_arr[mate_idx]].iter().map(|f| *f).collect();
        let parent_b: Vec<usize> = species[mating_arr[mate_idx + 1]].iter().map(|f| *f).collect();
        let child = cross_parent(&parent_a, &parent_b).unwrap();

        cross_result[cross_result_idx] = child;
        cross_result_idx += 1;
    }

    cross_result
} 

fn mutate(child: &Vec<usize>, mutation_prob: f64) -> Vec<usize> {
    let mut mutation_result = child.iter().map(|f| *f).collect();
    // Мутация - меняем случайный ген
    let mut rng = rand::rng();

    // Проверка на шанс мутации
    let mutation_chance = rng.random_range((0.)..(1.));
    if mutation_chance > mutation_prob {
        return mutation_result;
    }

    let gene_a_index = rng.random_range(0..mutation_result.len());
    let gene_b_index = rng.random_range(0..mutation_result.len());
    let tmp = child[gene_a_index];
    mutation_result[gene_a_index] = child[gene_b_index];
    mutation_result[gene_b_index] = tmp;
    return mutation_result;
}

// Создаём популяцию из случайных генов
fn create_population(genes_amount: usize, population_size: usize) -> Vec<Vec<usize>> {
    let mut rng = rand::rng();

    (0..population_size).map(|_| {
        let mut genes_array: Vec<usize> = (0..genes_amount).collect();

        for i in 0..genes_amount {
            let random_index = rng.random_range(0..genes_amount);
            let tmp = genes_array[i];
            genes_array[i] = genes_array[random_index];
            genes_array[random_index] = tmp;
        }

        genes_array
    }).collect()
}

// Выполняем селекцию
fn selection(species: Vec<Vec<usize>>, paths: &RwLockReadGuard<'_, Arc<dyn ACOPaths>>) -> Vec<Vec<usize>> {
    let mut selection_result = vec![vec![0 as usize; species[0].len()]; species.len()];
    let mut roulette = vec![0.; species.len()];
    let mut roulette_sum = 0.;

    /* Сгенерировать рулетку */
    for (specie_index, specie) in species.iter().enumerate() {
        let mut path_len = 0.;
        for gene_index in 0..(specie.len() - 1) {
            path_len += paths.get_distance(specie[gene_index], specie[gene_index + 1]).unwrap_or(0.);
        }
        path_len += paths.get_distance(*specie.last().unwrap(), specie[0]).unwrap_or(0.);
        
        let specie_score = 1. / path_len;
        roulette[specie_index] = specie_score;
        roulette_sum += specie_score;
    }

    // На основе полученной рулетки добавляем виды
    let mut specie_current_idx: usize = 0;
    for i in 0..species.len() {
        let specie_amount: usize = ((roulette[i] / roulette_sum) * species.len() as f64).round_ties_even() as usize;
        for _ in 0..specie_amount {
            if specie_current_idx >= selection_result.len() {
                println!("Warning! A specie overflow");
            }

            selection_result[specie_current_idx] = species[i].iter().map(|it| *it).collect();
            specie_current_idx += 1;
        }
    }

    while specie_current_idx < species.len() {
        selection_result[specie_current_idx] = selection_result[specie_current_idx - 1].iter().map(|it| *it).collect();
        specie_current_idx += 1;
    }

    selection_result
}

// На основе полученного массива путей сгенерировать следующую популяцию
fn next_generation(species: Vec<Vec<usize>>, paths: & mut Arc<dyn ACOPaths>, mate_prob: f64, mutation_prob: f64) -> Vec<Vec<usize>> {
    let field = Arc::new(RwLock::new(paths.clone()));
    let read_field = Arc::clone(&field);
    // Шаг 1: выполняем селекцию среди родительских генов
    let parent_generation = selection(species, &read_field.read().unwrap());
    // Шаг 2: скрещиваем между собой родителей
    let mut child_generation = cross(parent_generation, mate_prob);
    for i in 0..child_generation.len() {
        // Шаг 3: заставляем детей мутировать
        let child_mutate = mutate(&child_generation[i], mutation_prob);
        child_generation[i] = child_mutate;
    }

    // Возвращаем поколение думеров
    return child_generation;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross() {
        let arr_a = vec![1,5,0,4,6,2,3];
        let arr_b = vec![5,3,6,1,4,2,0];
        let child = cross_within_mixup_range(&arr_a, &arr_b, 3, 5).unwrap();
        assert_eq!(child, [5,4,6,1,3,2,0])
    }
}