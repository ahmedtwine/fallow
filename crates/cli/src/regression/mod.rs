mod baseline;
mod counts;
mod outcome;
mod tolerance;

#[allow(unused_imports, reason = "re-exports for lib.rs public API")]
pub use baseline::load_regression_baseline;
pub use baseline::{
    RegressionOpts, SaveRegressionTarget, compare_check_regression, save_baseline_to_config,
    save_regression_baseline,
};
pub use counts::CheckCounts;
#[allow(unused_imports, reason = "re-exports for lib.rs public API")]
pub use counts::{DupesCounts, RegressionBaseline};
pub use outcome::{RegressionOutcome, print_regression_outcome};
pub use tolerance::Tolerance;
