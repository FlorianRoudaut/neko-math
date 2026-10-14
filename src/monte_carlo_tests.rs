use crate::{Path, PathParameters, SeedableRng, StdRng, compute_stats, generate_path};

const SEED: u64 = 42;

fn params(n_steps: usize, mean: f64, std_dev: f64) -> PathParameters {
    PathParameters { n_steps, mean, std_dev }
}

#[test]
fn generate_path_with_no_steps_is_empty() {
    let mut rng = StdRng::seed_from_u64(SEED);
    assert!(generate_path(&mut rng, &params(0, 0.0, 1.0)).values.is_empty());
}

#[test]
fn generate_path_has_one_value_per_step() {
    let mut rng = StdRng::seed_from_u64(SEED);
    assert_eq!(generate_path(&mut rng, &params(1, 0.0, 1.0)).values.len(), 1);
    assert_eq!(generate_path(&mut rng, &params(100, 0.0, 1.0)).values.len(), 100);
}

#[test]
fn generate_path_with_zero_std_is_a_straight_line() {
    let mut rng = StdRng::seed_from_u64(SEED);
    let expected = Path { values: vec![0.5, 1.0, 1.5, 2.0] };
    assert_eq!(generate_path(&mut rng, &params(4, 0.5, 0.0)), expected);
}

#[test]
fn generate_path_with_same_seed_is_reproducible() {
    let path1 = generate_path(&mut StdRng::seed_from_u64(SEED), &params(1000, 0.0, 1.0));
    let path2 = generate_path(&mut StdRng::seed_from_u64(SEED), &params(1000, 0.0, 1.0));
    assert_eq!(path1, path2);
}

#[test]
fn generate_path_with_different_seeds_differs() {
    let path1 = generate_path(&mut StdRng::seed_from_u64(SEED), &params(1000, 0.0, 1.0));
    let path2 = generate_path(&mut StdRng::seed_from_u64(SEED + 1), &params(1000, 0.0, 1.0));
    assert_ne!(path1, path2);
}

#[test]
fn generate_path_increments_match_mean_and_std() {
    let (mean, std_dev) = (0.1, 2.0);
    let path = generate_path(&mut StdRng::seed_from_u64(SEED), &params(100_000, mean, std_dev)).values;
    let increments: Vec<f64> = std::iter::once(path[0])
        .chain(path.windows(2).map(|w| w[1] - w[0]))
        .collect();

    let stats = compute_stats(&increments).unwrap();
    // Standard error of the mean is std_dev / sqrt(n) ≈ 0.0063, allow ~5 of them
    assert!((stats.mean - mean).abs() < 0.03, "mean {} expected {}", stats.mean, mean);
    assert!((stats.std - std_dev).abs() < 0.03, "std {} expected {}", stats.std, std_dev);
}
