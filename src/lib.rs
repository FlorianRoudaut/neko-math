pub mod box_muller;
pub mod monte_carlo;
pub mod stats;

pub use box_muller::{box_muller, normal_variable};
pub use monte_carlo::generate_path;
pub use stats::{Stats, compute_stats, normal_cdf, normal_cdf_general, normal_inverse_cdf};
