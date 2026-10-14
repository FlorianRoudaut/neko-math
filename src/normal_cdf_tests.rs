use crate::normal_cdf::erf;
use crate::{erfc, normal_cdf, normal_inverse_cdf};

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

// Relative tolerance for the Cody approximations, accurate to about 1e-16
const CODY_TOLERANCE: f64 = 1e-14;

fn assert_close(actual: f64, expected: f64, label: &str) {
    let rel_error = ((actual - expected) / expected).abs();
    assert!(rel_error < CODY_TOLERANCE, "{} = {}, expected {} (relative error {})", label, actual, expected, rel_error);
}

#[test]
fn erf_matches_reference_values() {
    // Covers the |x| <= 0.46875, 0.46875 < x <= 4 regions
    assert_close(erf(0.1), 0.1124629160182849, "erf(0.1)");
    assert_close(erf(0.5), 0.5204998778130465, "erf(0.5)");
    assert_close(erf(1.0), 0.8427007929497149, "erf(1)");
    assert_close(erf(3.0), 0.9999779095030014, "erf(3)");
}

#[test]
fn erf_is_odd() {
    for x in [0.1, 0.4, 1.0, 3.0, 6.0] {
        assert_eq!(erf(-x), -erf(x), "erf(-{}) != -erf({})", x, x);
    }
}

#[test]
fn erfc_matches_reference_values() {
    // Covers negative x and the three Cody regions, down to values near the f64 underflow
    assert_close(erfc(-1.0), 1.842700792949715, "erfc(-1)");
    assert_close(erfc(0.3), 0.6713732405408726, "erfc(0.3)");
    assert_close(erfc(2.0), 0.004677734981047265, "erfc(2)");
    assert_close(erfc(5.0), 1.5374597944280351e-12, "erfc(5)");
    assert_close(erfc(10.0), 2.088487583762545e-45, "erfc(10)");
    assert_close(erfc(26.0), 5.663192408856143e-296, "erfc(26)");
    assert_eq!(erfc(30.0), 0.0);
}

#[test]
fn normal_cdf_matches_reference_values() {
    assert_eq!(normal_cdf(0.0), 0.5);
    assert_close(normal_cdf(1.96), 0.9750021048517796, "Φ(1.96)");
    assert_close(normal_cdf(-1.0), 0.15865525393145707, "Φ(-1)");
    // Deep tails keep their relative accuracy: no 1 - erf cancellation
    assert_close(normal_cdf(-5.0), 2.8665157187919455e-07, "Φ(-5)");
    assert_close(normal_cdf(-8.0), 6.220960574271819e-16, "Φ(-8)");
}

#[test]
fn normal_cdf_inverts_normal_inverse_cdf() {
    // Limited by Acklam's inverse: its ~1e-9 relative error on z grows to ~z * 1e-9 on p in the tails
    for p in [1e-10, 1e-5, 0.001, 0.02425, 0.1, 0.5, 0.9, 0.97575, 0.999] {
        let round_trip = normal_cdf(normal_inverse_cdf(p));
        assert!(((round_trip - p) / p).abs() < 1e-7, "Φ(Φ⁻¹({})) = {}", p, round_trip);
    }
}

// Breakpoints between the regions of Acklam's algorithm
const P_LOW: f64 = 0.02425;
const P_HIGH: f64 = 1.0 - P_LOW;

#[test]
fn normal_inverse_cdf_is_continuous_at_region_boundaries() {
    let eps = 1e-12;
    for p in [P_LOW, P_HIGH] {
        let below = normal_inverse_cdf(p - eps);
        let above = normal_inverse_cdf(p + eps);
        // The slope is about 1 / φ(2) ≈ 18, so the true gap is ~4e-11
        assert!((above - below).abs() < 1e-8, "jump at p = {}: {} vs {}", p, below, above);
    }
}

#[test]
fn normal_inverse_cdf_is_increasing() {
    let n = 100_000;
    let mut previous = f64::NEG_INFINITY;
    for i in 1..n {
        let p = i as f64 / n as f64;
        let z = normal_inverse_cdf(p);
        assert!(z > previous, "Φ⁻¹({}) = {} is not above the previous value {}", p, z, previous);
        previous = z;
    }
}
