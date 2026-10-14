use crate::normal_cdf::normal_cdf;

/// Calculates the probability that a Geometric Brownian Motion process 
/// hits barrier K before time T: P(\tau <= T). K can be above or below S_0.
///
/// Parameters:
/// - `s0`: Initial asset price S_0 (must be > 0)
/// - `k`: Limit price / barrier K (must be > 0)
/// - `t`: Time horizon T (must be > 0)
/// - `mu`: Drift coefficient \mu
/// - `sigma`: Volatility \sigma (must be > 0)
pub fn first_passage_probability(s0: f64, k: f64, t: f64, mu: f64, sigma: f64) -> f64 {
    assert!(s0 > 0.0, "Initial price s0 must be positive.");
    assert!(k > 0.0, "Barrier k must be positive.");
    assert!(t > 0.0, "Time t must be positive.");
    assert!(sigma > 0.0, "Volatility sigma must be positive.");

    let sigma_sq = sigma * sigma;
    let log_k_s0 = (k / s0).ln();
    // Drift of ln(S): nu = mu - 0.5 * sigma^2
    let nu = mu - 0.5 * sigma_sq;

    // A barrier below S0 is the mirror image of a barrier above it:
    // use the distance to the barrier and the drift towards it
    let distance = log_k_s0.abs();
    let drift_towards_barrier = if log_k_s0 >= 0.0 { nu } else { -nu };

    let drift_term = drift_towards_barrier * t;
    let denom = sigma * t.sqrt();

    // First term argument: (-|ln(K/S0)| + drift * T) / (sigma * sqrt(T))
    let d1 = (-distance + drift_term) / denom;

    // Second term argument: (-|ln(K/S0)| - drift * T) / (sigma * sqrt(T))
    let d2 = (-distance - drift_term) / denom;

    // Power factor: exp(2 * drift * |ln(K/S0)| / sigma^2), equal to (K / S0)^(2 * nu / sigma^2) in both directions
    let power_factor = (2.0 * drift_towards_barrier * distance / sigma_sq).exp();

    let prob = normal_cdf(d1) + power_factor * normal_cdf(d2);

    // Clamp value to [0.0, 1.0] to account for any floating-point precision error
    prob.clamp(0.0, 1.0)
}
