use rand::{self, Rng};

pub const FIELD_SIZE: usize = 20;
const MAX_ITERATIONS: usize = 10_000;

pub struct SimulatedAnnealingReport {
    pub best_field: [[bool; FIELD_SIZE]; FIELD_SIZE],
    pub bad_decisions: Vec<usize>,
    pub energy: Vec<usize>,
    pub temperatures: Vec<f64>
}

macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    }
}

pub fn pick_not_taken_place(field_of_queens: &[[bool; FIELD_SIZE]; FIELD_SIZE]) -> Result<(usize, usize), Box<&str>> {
    let mut rng = rand::rng();
    let mut current_iteration = 0;

    loop {
        let (inner_rand_i, inner_rand_j) = (rng.random_range(0..FIELD_SIZE), rng.random_range(0..FIELD_SIZE));
        if !field_of_queens[inner_rand_i][inner_rand_j] {
            return Ok((inner_rand_i, inner_rand_j));
        }
        current_iteration += 1;
        if current_iteration > MAX_ITERATIONS {
            return Err(Box::new("Failed to find not taken place"));
        }
    };
}

pub fn pick_taken_place(field_of_queens: &[[bool; FIELD_SIZE]; FIELD_SIZE]) -> Result<(usize, usize), Box<&str>> {
    let mut rng = rand::rng();
    let mut current_iteration = 0;

    loop {
        let (inner_rand_i, inner_rand_j) = (rng.random_range(0..FIELD_SIZE), rng.random_range(0..FIELD_SIZE));
        if field_of_queens[inner_rand_i][inner_rand_j] {
            return Ok((inner_rand_i, inner_rand_j));
        }
        current_iteration += 1;
        if current_iteration > MAX_ITERATIONS {
            return Err(Box::new("Failed to find taken place"));
        }
    };
}

/**
 * Генерирует поле с ферзями. Случайный подход - не самый лучший. Но так как мы обычно будем
 * работать с маленьким количеством ферзей, то не страшно
 */
pub fn generate_field(queens_amount: usize) -> [[bool; FIELD_SIZE]; FIELD_SIZE] {
    let mut field_of_queens = [[false; FIELD_SIZE]; FIELD_SIZE];

    for _ in (0..queens_amount) {
        match pick_not_taken_place(&field_of_queens) {
            Ok((rand_i, rand_j)) => {
                field_of_queens[rand_i][rand_j] = true;
            },
            Err(_err) => {
                continue;
            }
        }
    }

    field_of_queens
}

/**
 * Перемещает ферзя в случайное место
 */
pub fn move_random_queen(field_of_queens: &mut [[bool; FIELD_SIZE]; FIELD_SIZE]) {
    let (free_i, free_j) = unwrap_or_return!(pick_not_taken_place(&field_of_queens));
    let (taken_i, taken_j) = unwrap_or_return!(pick_taken_place(&field_of_queens));

    field_of_queens[free_i][free_j] = true;
    field_of_queens[taken_i][taken_j] = false;
}

pub fn compute_energy(field_of_queens: &[[bool; FIELD_SIZE]; FIELD_SIZE]) -> usize {
    let mut energy: usize = 0;
    
    for i in 0..FIELD_SIZE {
        for j in 0..FIELD_SIZE {
            if !field_of_queens[i][j] {
                continue;
            }

            for k in 0..FIELD_SIZE {
                if field_of_queens[k][j] && k != i {
                    energy += 1;
                }

                if field_of_queens[i][k] && k != j {
                    energy += 1;
                }
                
                if k != 0 {
                    if i >= k && j >= k && field_of_queens[i - k][j - k] {
                        energy += 1;
                    } 
                    if i + k < FIELD_SIZE && j >= k && field_of_queens[i + k][j - k] {
                        energy += 1;
                    } 
                    if i >= k && j + k < FIELD_SIZE && field_of_queens[i - k][j + k] {
                        energy += 1;
                    } 
                    if i + k < FIELD_SIZE && j + k < FIELD_SIZE && field_of_queens[i + k][j + k] {
                        energy += 1;
                    }
                }
            }
        }
    }

    energy
}

pub fn simulated_annealing(max_temp: f64,
                    min_temp: f64,
                    lower_coef: f64,
                    queen_amount: usize,
                    steps_amount: usize) -> SimulatedAnnealingReport {
    let mut rng = rand::rng();
    let mut current = generate_field(queen_amount);
    let mut current_energy = compute_energy(&current);
    let mut temp: f64 = max_temp;
    let mut report: SimulatedAnnealingReport = SimulatedAnnealingReport { 
        best_field: [[false; FIELD_SIZE]; FIELD_SIZE], 
        bad_decisions: vec![], 
        energy: vec![], 
        temperatures: vec![] 
    };

    while temp > min_temp {
        let mut bads: usize = 0;

        for _ in 0..steps_amount {
            let mut worker = current;
            move_random_queen(&mut worker);
            let worker_energy = compute_energy(&worker);

            if worker_energy < current_energy {
                current = worker;
                current_energy = worker_energy;
            } else {
                let delta = worker_energy - current_energy;
                let probability = (-(delta as f64) / temp).exp();
                if rng.random_range(0.0f64..1.0f64) < probability {
                    current = worker;
                    current_energy = worker_energy;
                    bads += 1;
                }
            }
        }

        report.temperatures.push(temp);
        report.bad_decisions.push(bads);
        report.energy.push(current_energy);
        temp *= lower_coef;
    }

    report.best_field = current;
    report
}