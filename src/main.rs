#![allow(non_snake_case)]

extern crate nalgebra as na;

mod parse;
mod solve;
mod util;

use na::{DMatrix, DVector};
use solve::SolveResult;

pub type Matrix = DMatrix<f64>;
pub type Vector = DVector<f64>;
pub const EPSILON: f64 = 1e-6;

fn main() -> Result<(), String> {
    let (args, flags): (Vec<String>, Vec<String>) =
        std::env::args().partition(|a| !a.starts_with("--"));

    let stdin = String::from("/dev/stdin");
    let path = args.get(1).unwrap_or(&stdin);

    let file_contents = parse::read_file(path)?;
    let parsed = parse::parse(&file_contents)?;

    let A = parsed.A;
    let b = parsed.b;
    let c = parsed.c;
    let N: Vec<usize> = (0..parsed.n).collect();
    let B: Vec<usize> = (parsed.n..parsed.n + parsed.m).collect();

    let solve_result = if !(b.min() < 0.0) {
        // Primal-feasible
        eprintln!("Solving primal problem");
        solve::primal(&A, &b, &c, B, N)?
    } else if !(c.max() > 0.0) {
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
    }

    println!("{}", solve_result);
    Ok(())
}
