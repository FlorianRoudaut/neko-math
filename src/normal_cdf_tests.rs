use crate::normal_inverse_cdf;

// Reference quantiles of N(0, 1)
const Z_001: f64 = 3.090232306167813; // Φ⁻¹(0.999)
const Z_01: f64 = 2.326347874040841;  // Φ⁻¹(0.99)
const TOLERANCE: f64 = 1e-6;

#[test]
fn normal_inverse_cdf_upper_tail() {
    assert!((normal_inverse_cdf(0.99) - Z_01).abs() < TOLERANCE);
    assert!((normal_inverse_cdf(0.999) - Z_001).abs() < TOLERANCE);
}

#[test]
fn normal_inverse_cdf_lower_tail() {
    let z = normal_inverse_cdf(0.01);
    assert!((z + Z_01).abs() < TOLERANCE, "Φ⁻¹(0.01) = {}, expected {}", z, -Z_01);
    let z = normal_inverse_cdf(0.001);
    assert!((z + Z_001).abs() < TOLERANCE, "Φ⁻¹(0.001) = {}, expected {}", z, -Z_001);
}

#[test]
fn normal_inverse_cdf_is_symmetric() {
    for p in [0.001, 0.01, 0.02, 0.1, 0.3] {
        let lower = normal_inverse_cdf(p);
        let upper = normal_inverse_cdf(1.0 - p);
        assert!((lower + upper).abs() < TOLERANCE, "Φ⁻¹({}) = {} but Φ⁻¹({}) = {}", p, lower, 1.0 - p, upper);
    }
}
