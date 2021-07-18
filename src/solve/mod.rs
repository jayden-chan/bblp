use crate::util::round_sig_figs;
use std::fmt;

mod dual;
mod primal;

pub use dual::*;
pub use primal::*;

/**
 * Number of significant figures to print in the results.
 * (not the number of sig figs used in the calculations)
 */
const PRINT_SIG_FIGS: u32 = 7;

/**
 * Represents an optimal solution to a linear program
 */
pub struct Solution {
    objective_value: f64,
    variable_values: Vec<f64>,
    pub pivots: usize,
    pub B: Vec<usize>,
    pub N: Vec<usize>,
}

/**
 * The possible outcomes of attempting to run
 * the simplex method on a given linear program
 */
pub enum SolveResult {
    Infeasible,
    Unbounded,
    Optimal(Solution),
}

/**
 * Format the results for submission
 */
impl fmt::Display for SolveResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolveResult::Infeasible => write!(f, "infeasible"),
            SolveResult::Unbounded => write!(f, "unbounded"),
            SolveResult::Optimal(results) => {
                let x_vals = results
                    .variable_values
                    .iter()
                    .map(|v| format!("{}", round_sig_figs(*v, PRINT_SIG_FIGS)))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(
                    f,
                    "optimal\n{}\n{}",
                    round_sig_figs(results.objective_value, PRINT_SIG_FIGS),
                    x_vals
                )
            }
        }
    }
}

/**
 * Format the results for debugging
 */
impl fmt::Debug for SolveResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolveResult::Infeasible => write!(f, "infeasible"),
            SolveResult::Unbounded => write!(f, "unbounded"),
            SolveResult::Optimal(results) => {
                let x_vals = results
                    .variable_values
                    .iter()
                    .map(|v| format!("{}", round_sig_figs(*v, PRINT_SIG_FIGS)))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(
                    f,
                    "{} pivots\noptimal\n{}\n{}",
                    results.pivots,
                    round_sig_figs(results.objective_value, PRINT_SIG_FIGS),
                    x_vals
                )
            }
        }
    }
}
