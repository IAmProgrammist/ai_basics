use std::error::Error;

use rand::Rng;

use crate::utils::ACOPaths;

pub struct MatrixACOPaths {
    feromone: Vec<Vec<f64>>,
    weights: Vec<Vec<f64>>
}

impl MatrixACOPaths {
    pub fn new(points_amount: usize) -> MatrixACOPaths {
        let mut rng = rand::rng();

        let mut weights: Vec<Vec<f64>> = vec![vec![0.; points_amount]; points_amount];
        for i in 0..points_amount {
            for j in 0..points_amount {
                if i == j {
                    continue;
                }

                weights[i][j] = rng.random_range(1.0..5.0)
            }
        }
        
        MatrixACOPaths {
            feromone: vec![vec![0.; points_amount]; points_amount],
            weights,
        }
    }
}

impl ACOPaths for MatrixACOPaths {
    fn len(&self) -> usize {
        self.weights.len()
    }

    fn get_distance(&self, from: usize, to: usize) -> Result<f64, Box<dyn Error>> {
        if from >= self.len() || to >= self.len() {
            return Err("Out of bounds".into());
        }

        return Ok(self.weights[from][to])
    }

    fn get_feromone_intensity(&self, from: usize, to: usize) -> Result<f64, Box<dyn Error>> {
        if from >= self.len() || to >= self.len() {
            return Err("Out of bounds".into());
        }

        return Ok(self.feromone[from][to])
    }

    fn set_feromone_intensity(&mut self, value: f64, from: usize, to: usize) -> Result<(), Box<dyn Error>> {
        if from >= self.len() || to >= self.len() {
            return Err("Out of bounds".into());
        }

        self.feromone[from][to] = value;
        Ok(())
    }
}