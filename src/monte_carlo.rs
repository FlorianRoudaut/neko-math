use crate::box_muller::normal_variable;

pub fn generate_path(n_steps: usize, mean: f64, std_dev: f64) -> Vec<f64> {
    let mut path = vec![0.0; n_steps];
    path[0] = normal_variable(mean, std_dev);
    for i in 1..n_steps {
        path[i] = path[i-1] + normal_variable(mean, std_dev);
    }
    path
}
