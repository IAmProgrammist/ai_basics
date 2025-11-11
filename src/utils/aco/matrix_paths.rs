use std::{error::Error, ops::Range, sync::Arc};

use rand::{distr::uniform::SampleRange, Rng};

use crate::utils::ACOPaths;


#[derive(Clone, PartialEq)]
pub struct Point2 {
    pub x: f64, 
    pub y: f64
}

impl Point2 {
    pub fn dist(&self, other: &Point2) -> f64 {
        ((self.x - other.x).powf(2.) + (self.y - other.y).powf(2.)).sqrt()
    }
}

#[derive(Clone, PartialEq)]
pub struct MatrixACOPaths {
    feromone: Vec<Vec<f64>>,
    pub points: Vec<Point2>,
    fresh: bool
}

const DISTR_X: f64 = 10.;
const DISTR_Y: f64 = 10.;

impl MatrixACOPaths {
    pub fn new(points_amount: usize, x_range: Option<Range<f64>>, y_range: Option<Range<f64>>) -> MatrixACOPaths {
        let x_range = x_range.unwrap_or(-DISTR_X..DISTR_X);
        let y_range = y_range.unwrap_or(-DISTR_Y..DISTR_Y);
        
        let mut rng = rand::rng();

        MatrixACOPaths {             
            feromone: vec![vec![1. / points_amount as f64; points_amount]; points_amount], 
            points: (0..points_amount).step_by(1).into_iter()
                .map(|_| Point2 {x: rng.random_range(x_range.clone()), y: rng.random_range(y_range.clone())}).collect(),
            fresh: true
        }
    }

    pub fn copy_fresh_and_feromone_from_trait(mut self, paths: & mut Arc<dyn ACOPaths>) -> MatrixACOPaths {
        let points_amount = paths.len();

        if points_amount != self.len() {
            return self
        }

        self.fresh = paths.is_fresh();

        for i in 0..points_amount {
            for j in 0..points_amount {
                self.set_feromone_intensity(paths.get_feromone_intensity(i, j).unwrap_or(0.), i, j).unwrap()
            }
        }

        self
    }

    pub fn clean_feromone(&mut self) {
        let points_amount= self.points.len();
        self.feromone = vec![vec![1. / points_amount as f64; points_amount]; points_amount];
        self.fresh = true;
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
        self.fresh = false;
        Ok(())
    }

    fn is_fresh(&self) -> bool {
        self.fresh
    }
}
