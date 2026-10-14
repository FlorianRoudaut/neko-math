use crate::box_muller::normal_variable;

pub fn generate_path(n_steps: usize, mean: f64, std_dev: f64) -> Vec<f64> {
    let mut path = Vec::with_capacity(n_steps);
    let mut level = 0.0;
    for _ in 0..n_steps {
        level += normal_variable(mean, std_dev);
        path.push(level);
    }
    path
}
