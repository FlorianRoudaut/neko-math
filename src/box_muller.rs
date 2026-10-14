use std::f64::consts::PI;
use rand::{RngExt, rngs::StdRng};

pub fn box_muller(rng: &mut StdRng) -> (f64, f64) {
    // Generate u1 in range (0, 1) to avoid ln(0)
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

pub fn normal_variable(rng: &mut StdRng, mean: f64, std_dev: f64) -> f64 {
    let (z0, _) = box_muller(rng);
    mean + z0 * std_dev
}
