use std::error::Error;

pub trait ACOPaths: Send + Sync {
    // Возвращает количество точек для карты
    fn len(&self) -> usize;

    // Возвращает расстояние между точками
    fn get_distance(&self, from: usize, to: usize) -> Result<f64, Box<dyn Error>>;

    // Возвращает количество феромона между точками
    fn get_feromone_intensity(&self, from: usize, to: usize) -> Result<f64, Box<dyn Error>>;

    // Устанавливает количество феромона между точками
    fn set_feromone_intensity(&mut self, value: f64, from: usize, to: usize) -> Result<(), Box<dyn Error>>;

    // Возвращает, нет ли на поле феромона
    fn is_fresh(&self) -> bool;
}
