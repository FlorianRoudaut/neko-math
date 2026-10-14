use crate::{MonteCarloParameters, first_passage_probability, generate_paths, normal_cdf};

const S0: f64 = 100.0;
const SEED: u64 = 42;

/// Fraction of simulated log-price paths that reach log(k / S0) at one of the steps.
/// The paths start at log(S0) = 0 and add N(log_drift, sigma^2) increments per step.
fn simulated_hit_probability(k: f64, steps: usize, log_drift: f64, sigma: f64, paths: usize) -> f64 {
    let barrier = (k / S0).ln();
    let batch_size = 1000;
    let mut hits = 0;
    // Batched to keep memory bounded
    for batch in 0..paths / batch_size {
        let params = MonteCarloParameters {
            paths_number: batch_size,
            steps_number: steps,
            mean: log_drift,
            std_dev: sigma,
            seed: SEED + batch as u64,
        };
        hits += generate_paths(&params)
            .iter()
            .filter(|p| p.returns.iter().any(|&r| if barrier > 0.0 { r >= barrier } else { r <= barrier }))
            .count();
    }
    hits as f64 / (paths / batch_size * batch_size) as f64
}

/// Checking the barrier only at discrete steps misses crossings between them.
/// Broadie-Glasserman: a discretely monitored barrier behaves like a continuous one
/// moved away from the start by 0.5826 * sigma * sqrt(dt), with dt = 1 here.
fn continuity_corrected_barrier(k: f64, sigma: f64) -> f64 {
    const BETA: f64 = 0.5826;
    let shift = BETA * sigma;
    if k > S0 { k * shift.exp() } else { k * (-shift).exp() }
}

fn assert_matches_simulation(k: f64, log_drift: f64) {
    let (steps, paths) = (1000, 5_000);
    // Total volatility over the horizon is 0.1
    let sigma = 0.1 / (steps as f64).sqrt();
    // A GBM with drift mu has log-drift mu - sigma^2 / 2
    let mu = log_drift + 0.5 * sigma * sigma;

    let simulated = simulated_hit_probability(k, steps, log_drift, sigma, paths);
    let closed_form = first_passage_probability(S0, continuity_corrected_barrier(k, sigma), steps as f64, mu, sigma);

    let standard_error = (closed_form * (1.0 - closed_form) / paths as f64).sqrt();
    assert!(
        (simulated - closed_form).abs() < 4.0 * standard_error,
        "K = {}, log-drift = {}: simulated {} vs closed form {} (standard error {})",
        k, log_drift, simulated, closed_form, standard_error
    );
}

#[test]
fn first_passage_probability_matches_simulation_without_drift() {
    assert_matches_simulation(105.0, 0.0);
}

#[test]
fn first_passage_probability_matches_simulation_with_drift() {
    // Log-drift of +0.02 over the whole horizon
    assert_matches_simulation(105.0, 0.02 / 1000.0);
}

#[test]
fn first_passage_probability_for_barrier_below_start() {
    // Without log-drift, the reflection principle gives P(min <= K) = 2 * Φ(ln(K / S0) / (sigma * sqrt(T)))
    let (k, t, sigma): (f64, f64, f64) = (95.0, 1.0, 0.1);
    let mu = 0.5 * sigma * sigma;
    let expected = 2.0 * normal_cdf((k / S0).ln() / (sigma * t.sqrt()));
    let actual = first_passage_probability(S0, k, t, mu, sigma);
    assert!((actual - expected).abs() < 1e-12, "P(hit {} from {}) = {}, expected {}", k, S0, actual, expected);
}

#[test]
fn first_passage_probability_for_barrier_below_start_matches_simulation() {
    assert_matches_simulation(95.0, 0.0);
}
