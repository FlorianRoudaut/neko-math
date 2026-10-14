
#[derive(Debug)]
pub struct Stats {
    pub mean: f64,
    pub std: f64,
}

pub fn compute_stats(values: &[f64]) -> Option<Stats> {
    let n = values.len();
    if n < 2 {
        return None;
    }

    let mut sum = 0.0;
    for &v in values {
        sum += v;
    }
    let mean = sum / n as f64;

    let mut variance_sum = 0.0;
    for &v in values {
        let diff = v - mean;
        variance_sum += diff * diff;
    }
    let std = (variance_sum / (n - 1) as f64).sqrt();

    Some(Stats { mean, std })
}
