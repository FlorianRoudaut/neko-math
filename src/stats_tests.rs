use crate::compute_stats;

const TOLERANCE: f64 = 1e-12;

fn assert_close(actual: f64, expected: f64, label: &str) {
    assert!((actual - expected).abs() < TOLERANCE, "{} = {}, expected {}", label, actual, expected);
}

#[test]
fn compute_stats_on_known_sample() {
    let stats = compute_stats(&[2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]).unwrap();
    assert_close(stats.mean, 5.0, "mean");
    // Sum of squared deviations is 32, sample std divides by n - 1
    assert_close(stats.std, (32.0_f64 / 7.0).sqrt(), "std");
}

#[test]
fn compute_stats_needs_at_least_two_values() {
    assert!(compute_stats(&[]).is_none());
    assert!(compute_stats(&[1.0]).is_none());

    let stats = compute_stats(&[1.0, 3.0]).unwrap();
    assert_close(stats.mean, 2.0, "mean");
    assert_close(stats.std, 2.0_f64.sqrt(), "std");
}

#[test]
fn compute_stats_of_constant_series_has_zero_std() {
    let stats = compute_stats(&[4.2; 10]).unwrap();
    assert_close(stats.mean, 4.2, "mean");
    // Not exactly 0: 4.2 is not representable, so the computed mean is off by a rounding error
    assert_close(stats.std, 0.0, "std");
}

#[test]
fn compute_stats_shift_moves_mean_only() {
    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let shifted: Vec<f64> = values.iter().map(|v| v + 10.0).collect();
    let stats = compute_stats(&values).unwrap();
    let shifted_stats = compute_stats(&shifted).unwrap();
    assert_close(shifted_stats.mean, stats.mean + 10.0, "shifted mean");
    assert_close(shifted_stats.std, stats.std, "shifted std");
}

#[test]
fn compute_stats_scale_multiplies_mean_and_std() {
    let values = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let scaled: Vec<f64> = values.iter().map(|v| v * 3.0).collect();
    let stats = compute_stats(&values).unwrap();
    let scaled_stats = compute_stats(&scaled).unwrap();
    assert_close(scaled_stats.mean, stats.mean * 3.0, "scaled mean");
    assert_close(scaled_stats.std, stats.std * 3.0, "scaled std");
}

#[test]
fn compute_stats_is_accurate_with_large_offset() {
    // A one-pass sum of squares would lose the ±1 deviations next to 1e18
    let values = [1e9 + 1.0, 1e9 - 1.0, 1e9 + 1.0, 1e9 - 1.0];
    let stats = compute_stats(&values).unwrap();
    assert_eq!(stats.mean, 1e9);
    assert_close(stats.std, (4.0_f64 / 3.0).sqrt(), "std");
}
