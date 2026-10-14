use crate::{MonteCarloParameters, Path, compute_stats, generate_paths};

const SEED: u64 = 42;

fn params(paths_number: usize, steps_number: usize, mean: f64, std_dev: f64) -> MonteCarloParameters {
    MonteCarloParameters { paths_number, steps_number, mean, std_dev, seed: SEED, batch_size: 10 }
}

/// Collects all the batches into one Vec
fn paths(seed: u64, params: &MonteCarloParameters) -> Vec<Path> {
    let mut paths = Vec::new();
    generate_paths(&MonteCarloParameters { seed, ..*params }, |batch| paths.extend(batch));
    paths
}

#[test]
fn generate_paths_returns_n_paths_of_n_steps() {
    let paths = paths(SEED, &params(5, 10, 0.0, 1.0));
    assert_eq!(paths.len(), 5);
    assert!(paths.iter().all(|p| p.returns.len() == 10));
}

#[test]
fn generate_paths_with_no_paths_is_empty() {
    assert!(paths(SEED, &params(0, 10, 0.0, 1.0)).is_empty());
}

#[test]
fn generate_paths_with_no_steps_gives_empty_paths() {
    let paths = paths(SEED, &params(3, 0, 0.0, 1.0));
    assert_eq!(paths.len(), 3);
    assert!(paths.iter().all(|p| p.returns.is_empty()));
}

#[test]
fn generate_paths_with_zero_std_are_straight_lines() {
    let expected = Path { returns: vec![0.5, 1.0, 1.5, 2.0] };
    assert!(paths(SEED, &params(3, 4, 0.5, 0.0)).iter().all(|p| *p == expected));
}

#[test]
fn generate_paths_gives_different_paths() {
    let paths = paths(SEED, &params(2, 100, 0.0, 1.0));
    assert_ne!(paths[0], paths[1]);
}

#[test]
fn generate_paths_with_same_seed_is_reproducible() {
    let params = params(3, 100, 0.0, 1.0);
    assert_eq!(paths(SEED, &params), paths(SEED, &params));
}

#[test]
fn generate_paths_with_different_seeds_differs() {
    let params = params(3, 100, 0.0, 1.0);
    assert_ne!(paths(SEED, &params), paths(SEED + 1, &params));
}

#[test]
fn generate_paths_increments_match_mean_and_std() {
    let (mean, std_dev) = (0.1, 2.0);
    let increments: Vec<f64> = paths(SEED, &params(100, 1000, mean, std_dev))
        .iter()
        .flat_map(|p| {
            std::iter::once(p.returns[0]).chain(p.returns.windows(2).map(|w| w[1] - w[0]))
        })
        .collect();

    let stats = compute_stats(&increments).unwrap();
    // Standard error of the mean is std_dev / sqrt(n) ≈ 0.0063, allow ~5 of them
    assert!((stats.mean - mean).abs() < 0.03, "mean {} expected {}", stats.mean, mean);
    assert!((stats.std - std_dev).abs() < 0.03, "std {} expected {}", stats.std, std_dev);
}

#[test]
fn generate_paths_splits_into_batches_of_batch_size() {
    let params = MonteCarloParameters { batch_size: 4, ..params(10, 5, 0.0, 1.0) };
    let mut batch_lens = Vec::new();
    generate_paths(&params, |batch| batch_lens.push(batch.len()));
    assert_eq!(batch_lens, vec![4, 4, 2]);
}

#[test]
fn generate_paths_does_not_depend_on_batch_size() {
    let one_batch = paths(SEED, &MonteCarloParameters { batch_size: 100, ..params(10, 50, 0.0, 1.0) });
    let small_batches = paths(SEED, &MonteCarloParameters { batch_size: 3, ..params(10, 50, 0.0, 1.0) });
    assert_eq!(one_batch, small_batches);
}

#[test]
#[should_panic(expected = "Batch size must be positive")]
fn generate_paths_with_zero_batch_size_panics() {
    generate_paths(&MonteCarloParameters { batch_size: 0, ..params(10, 5, 0.0, 1.0) }, |_| {});
}
