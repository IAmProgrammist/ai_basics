#[derive(Clone, Copy)]
pub struct ACOConfig {
    pub ants: usize,
    pub feromone_weight: f64,
    pub heuristic_coefficient: f64,
    pub q: f64,
    pub target_city: usize,
    pub evaporation_coefficient: f64
}
