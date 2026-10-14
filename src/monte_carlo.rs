use rand::Rng;
use crate::box_muller::normal_variable;

/// Parameters of a random walk: each step adds a N(mean, std_dev^2) increment
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PathParameters {
    pub n_steps: usize,
    pub mean: f64,
    pub std_dev: f64,
}

/// Simulated path: the level of the walk after each step
#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub values: Vec<f64>,
}

pub fn generate_path<R: Rng + ?Sized>(rng: &mut R, params: &PathParameters) -> Path {
    let mut values = Vec::with_capacity(params.n_steps);
    let mut level = 0.0;
    for _ in 0..params.n_steps {
        level += normal_variable(rng, params.mean, params.std_dev);
        values.push(level);
    }
    Path { values }
}
