use std::error::Error;

use rand::Rng;

use crate::utils::ACOPaths;


#[derive(Clone)]
pub struct Point2 {
    x: f64, y: f64
}

impl Point2 {
    pub fn dist(&self, other: &Point2) -> f64 {
        (self.x - other.x).powf(2.) + (self.y - other.y).powf(2.)
    }
}

pub struct MatrixACOPaths {
    feromone: Vec<Vec<f64>>,
    points: Vec<Point2>
}

const DISTR_X: f64 = 10.;
const DISTR_Y: f64 = 10.;

impl MatrixACOPaths {
    pub fn new(points_amount: usize) -> MatrixACOPaths {
        let mut rng = rand::rng();

        MatrixACOPaths {             
            feromone: vec![vec![0.; points_amount]; points_amount], 
            points: (0..points_amount).step_by(1).into_iter()
                .map(|_| Point2 {x: rng.random_range(-DISTR_X..DISTR_X), y: rng.random_range(-DISTR_Y..DISTR_Y)}).collect()
        }
    }
}

impl ACOPaths for MatrixACOPaths {
    fn len(&self) -> usize {
        self.points.len()
    }

    fn get_distance(&self, from: usize, to: usize) -> Result<f64, Box<dyn Error>> {
        if from >= self.len() || to >= self.len() {
            return Err("Out of bounds".into());
        }

        return Ok(self.points[from].dist(&self.points[to]))
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