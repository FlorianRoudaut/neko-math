use std::f64::consts::PI;
use rand::{RngExt, rng};

pub fn box_muller() -> (f64, f64) {
    let mut rng = rng();
    // Generate u1 in range (0, 1] to avoid ln(0)
    let u1 = loop {
        let val = rng.random::<f64>();
        if val > 0.0 {
            break val;
        }
    };
    let u2 = rng.random::<f64>();

    let r = (-2.0_f64 * u1.ln()).sqrt();
    let theta = 2.0 * PI * u2;

    let z0 = r * theta.cos();
    let z1 = r * theta.sin();

    (z0, z1)
}

pub fn normal_variable(mean: f64, std_dev: f64) -> f64 {
    let (z0, _) = box_muller();
    mean + z0 * std_dev
}
