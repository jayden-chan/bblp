#![allow(non_snake_case)]
#![allow(clippy::neg_cmp_op_on_partial_ord)]
#![allow(clippy::many_single_char_names)]

extern crate nalgebra as na;

mod parse;
mod solve;
mod util;

use na::{DMatrix, DVector};
use solve::SolveResult;

pub type Matrix = DMatrix<f64>;
pub type Vector = DVector<f64>;

/**
 * Value used instead of 0 for checking when variables
 * are negative/non-negative. This value was previously
 * used to mitigate floating point problems with many
 * of the netlib tests. I'm not sure if it's still
 * necessary after implementing perturbation but I'm
 * leaving it in anyway.
 */
pub const EPSILON: f64 = 1e-9;

/**
 * Perturbation amount copied from glpk source code.
 * I'm not sure if there is a perscribed way for choosing
 * this value other than for it to be "sufficiently small".
 */
pub const PERTURB_AMT: f64 = 1e-9;

fn main() -> Result<(), String> {
    let (args, flags): (Vec<String>, Vec<String>) =
        std::env::args().skip(1).partition(|a| !a.starts_with("--"));

    let stdin = String::from("/dev/stdin");
    let path = args.get(0).unwrap_or(&stdin);

    let file_contents = parse::read_file(path)?;
    let parsed = parse::parse(&file_contents)?;

    let A = parsed.A;
    let b = parsed.b;
    let c = parsed.c;
    let N: Vec<usize> = (0..parsed.n).collect();
    let B: Vec<usize> = (parsed.n..parsed.n + parsed.m).collect();

    let solve_result = if !(b.min() < -f64::EPSILON) {
        // Primal-feasible
        eprintln!("Solving primal problem");
        solve::primal(&A, &b, &c, B, N)?
    } else if !(c.max() > f64::EPSILON) {
        // Dual-feasible
        eprintln!("Solving dual problem");
        solve::dual(&A, &b, &c, B, N)?
    } else {
        let zero = Vector::zeros(c.len());
        eprintln!("Solving aux problem");

        match solve::dual(&A, &b, &zero, B, N)? {
            SolveResult::Optimal(aux_solution) => {
                solve::primal(&A, &b, &c, aux_solution.B, aux_solution.N)?
            }
            SolveResult::Unbounded | SolveResult::Infeasible => {
                SolveResult::Infeasible
            }
        }
    };

    if flags.iter().any(|flag| flag == "--debug") {
        eprintln!("{:?}", solve_result);
    } else {
        println!("{}", solve_result);
    }

    Ok(())
}
