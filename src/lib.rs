pub mod box_muller;
pub mod monte_carlo;
pub mod normal_cdf;
pub mod stats;
#[cfg(test)]
mod normal_cdf_tests;
#[cfg(test)]
mod monte_carlo_tests;

pub use box_muller::{box_muller, normal_variable};
pub use monte_carlo::{MonteCarloParameters, Path, generate_paths};
pub use normal_cdf::{normal_cdf, normal_cdf_general, normal_inverse_cdf};
pub use stats::{Stats, compute_stats};
pub use rand::{Rng, SeedableRng, rngs::StdRng};
