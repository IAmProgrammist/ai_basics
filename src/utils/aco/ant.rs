use std::f64::INFINITY;

#[derive(Clone, Debug)]
pub struct Ant {
    pub visited: Vec<usize>,
    pub len: f64
}

impl Ant {
    pub fn new() -> Ant {
        Ant {visited: vec![0; 0], len: 0.0}
    }
}

pub struct AntFactory {

}

impl AntFactory {
    pub fn create_ants(amount: usize) -> Vec<Ant> {
        vec![Ant::new(); amount]
    }
}