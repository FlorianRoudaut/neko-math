use crate::normal_cdf::normal_cdf;

/// Calculates the probability that a Geometric Brownian Motion process 
/// hits barrier K before time T: P(\tau <= T).
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

    // Trivial case: barrier already hit/exceeded at t = 0
    if (k >= s0 && s0 >= k) || (k > s0 && s0 >= k) || (k < s0 && s0 <= k) {
        // Handle floating point exact equality or boundary start
    }
    
    let sigma_sq = sigma * sigma;
    let sqrt_t = t.sqrt();
    let log_k_s0 = (k / s0).ln();
    
    let drift_term = (mu - 0.5 * sigma_sq) * t;
    let denom = sigma * sqrt_t;

    // First term argument: (-ln(K/S0) + (mu - 0.5 * sigma^2)*T) / (sigma * sqrt(T))
    let d1 = (-log_k_s0 + drift_term) / denom;
    
    // Second term argument: (-ln(K/S0) - (mu - 0.5 * sigma^2)*T) / (sigma * sqrt(T))
    let d2 = (-log_k_s0 - drift_term) / denom;

    // Power factor: (K / S0)^(2*mu / sigma^2 - 1)
    let power_exponent = (2.0 * mu / sigma_sq) - 1.0;
    let power_factor = (k / s0).powf(power_exponent);

    let prob = normal_cdf(d1) + power_factor * normal_cdf(d2);

    // Clamp value to [0.0, 1.0] to account for any floating-point precision error
    prob.clamp(0.0, 1.0)
}
