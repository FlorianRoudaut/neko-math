
/// Cumulative Distribution Function for N(mean, std_dev^2)
pub fn normal_cdf_general(x: f64, mean: f64, std_dev: f64) -> f64 {
    normal_cdf((x - mean) / std_dev)
}

/// Standard Normal Cumulative Distribution Function Φ(x)
/// Returns P(X <= x) for a standard normal distribution N(0, 1)
pub fn normal_cdf(x: f64) -> f64 {
    0.5 * erfc(-x / std::f64::consts::SQRT_2)
}

/// Error function erf(x), using W. J. Cody's rational approximations
/// (Math. Comp. 23, 1969). Accurate to about 1e-16.
// Only used by the tests for now: normal_cdf goes through erfc for its tail accuracy
#[allow(dead_code)]
pub(crate) fn erf(x: f64) -> f64 {
    if x.abs() <= ERF_SMALL_LIMIT {
        erf_small(x)
    } else {
        // Computed on |x| so that erf stays exactly odd
        (1.0 - erfc_positive(x.abs())).copysign(x)
    }
}

/// Complementary error function erfc(x) = 1 - erf(x), using W. J. Cody's rational approximations.
/// Computed directly, so it keeps its relative accuracy in the tails where 1 - erf(x) would cancel.
pub fn erfc(x: f64) -> f64 {
    if x < 0.0 {
        2.0 - erfc_positive(-x)
    } else {
        erfc_positive(x)
    }
}

const ERF_SMALL_LIMIT: f64 = 0.46875;
const ERFC_MID_LIMIT: f64 = 4.0;
/// Above this erfc(x) underflows to 0
const ERFC_ZERO_LIMIT: f64 = 26.543;

/// erf(x) for |x| <= 0.46875
fn erf_small(x: f64) -> f64 {
    const A: [f64; 5] = [
        3.16112374387056560e+00,
        1.13864154151050156e+02,
        3.77485237685302021e+02,
        3.20937758913846947e+03,
        1.85777706184603153e-01,
    ];
    const B: [f64; 4] = [
        2.36012909523441209e+01,
        2.44024637934444173e+02,
        1.28261652607737228e+03,
        2.84423683343917062e+03,
    ];

    let x_sq = x * x;
    let mut num = A[4] * x_sq;
    let mut den = x_sq;
    for i in 0..3 {
        num = (num + A[i]) * x_sq;
        den = (den + B[i]) * x_sq;
    }
    x * (num + A[3]) / (den + B[3])
}

/// erfc(x) for x >= 0
fn erfc_positive(x: f64) -> f64 {
    if x <= ERF_SMALL_LIMIT {
        1.0 - erf_small(x)
    } else if x <= ERFC_MID_LIMIT {
        exp_minus_x_sq(x) * erfc_mid_ratio(x)
    } else if x < ERFC_ZERO_LIMIT {
        exp_minus_x_sq(x) * erfc_large_ratio(x)
    } else {
        0.0
    }
}

/// erfc(x) / exp(-x^2) for 0.46875 < x <= 4
fn erfc_mid_ratio(x: f64) -> f64 {
    const C: [f64; 9] = [
        5.64188496988670089e-01,
        8.88314979438837594e+00,
        6.61191906371416295e+01,
        2.98635138197400131e+02,
        8.81952221241769090e+02,
        1.71204761263407058e+03,
        2.05107837782607147e+03,
        1.23033935479799725e+03,
        2.15311535474403846e-08,
    ];
    const D: [f64; 8] = [
        1.57449261107098347e+01,
        1.17693950891312499e+02,
        5.37181101862009858e+02,
        1.62138957456669019e+03,
        3.29079923573345963e+03,
        4.36261909014324716e+03,
        3.43936767414372164e+03,
        1.23033935480374942e+03,
    ];

    let mut num = C[8] * x;
    let mut den = x;
    for i in 0..7 {
        num = (num + C[i]) * x;
        den = (den + D[i]) * x;
    }
    (num + C[7]) / (den + D[7])
}

/// erfc(x) / exp(-x^2) for x > 4
fn erfc_large_ratio(x: f64) -> f64 {
    const P: [f64; 6] = [
        3.05326634961232344e-01,
        3.60344899949804439e-01,
        1.25781726111229246e-01,
        1.60837851487422766e-02,
        6.58749161529837803e-04,
        1.63153871373020978e-02,
    ];
    const Q: [f64; 5] = [
        2.56852019228982242e+00,
        1.87295284992346725e+00,
        5.27905102951428412e-01,
        6.05183413124413191e-02,
        2.33520497626869185e-03,
    ];
    const ONE_OVER_SQRT_PI: f64 = 5.6418958354775628695e-01;

    let inv_x_sq = 1.0 / (x * x);
    let mut num = P[5] * inv_x_sq;
    let mut den = inv_x_sq;
    for i in 0..4 {
        num = (num + P[i]) * inv_x_sq;
        den = (den + Q[i]) * inv_x_sq;
    }
    let r = inv_x_sq * (num + P[4]) / (den + Q[4]);
    (ONE_OVER_SQRT_PI - r) / x
}

/// exp(-x^2), split as exp(-x_hi^2) * exp(-(x - x_hi)(x + x_hi)) to limit the rounding error of x^2
fn exp_minus_x_sq(x: f64) -> f64 {
    let x_hi = (x * 16.0).trunc() / 16.0;
    let delta = (x - x_hi) * (x + x_hi);
    (-x_hi * x_hi).exp() * (-delta).exp()
}

/// Standard Normal Inverse Cumulative Distribution Function (probit)
/// Approximates z = Φ⁻¹(p) for 0 < p < 1 using Acklam's algorithm.
pub fn normal_inverse_cdf(p: f64) -> f64 {
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if p >= 1.0 {
        return f64::INFINITY;
    }

    // Coefficients for central region
    const A: [f64; 6] = [
        -3.969683028665376e+01,
         2.209460984245205e+02,
        -2.759285104469687e+02,
         1.383577518672690e+02,
        -3.066479806614716e+01,
         2.506628277459239e+00,
    ];
    const B: [f64; 5] = [
        -5.447609879822406e+01,
         1.615858368580409e+02,
        -1.556989798598866e+02,
         6.680131188771972e+01,
        -1.328068155288572e+01,
    ];

    // Coefficients for tail regions
    const C: [f64; 6] = [
        -7.784894002430293e-03,
        -3.223964580411365e-01,
        -2.400758277161838e+00,
        -2.549732539343734e+00,
         4.374664141464968e+00,
         2.938163982698783e+00,
    ];
    const D: [f64; 4] = [
         7.784695709041462e-03,
         3.224671290700398e-01,
         2.445134137142996e+00,
         3.754408661907416e+00,
    ];

    // Breakpoints
    const P_LOW: f64 = 0.02425;
    const P_HIGH: f64 = 1.0 - P_LOW;

    if p < P_LOW {
        // Lower tail region
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= P_HIGH {
        // Central region
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        // Upper tail region
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5])
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    }
}

