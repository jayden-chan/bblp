#![allow(non_snake_case)]

extern crate nalgebra as na;

use crate::solve::{solve_dual, solve_primal, Solution};
use na::DVector;

pub const EPSILON: f64 = 0.0000001;

mod parse;
mod solve;
mod util;

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
    let n = parsed.n;
    let m = parsed.m;
    let N: Vec<usize> = (0..parsed.n).collect();
    let B: Vec<usize> = (parsed.n..parsed.n + parsed.m).collect();

    if flags.iter().any(|f| f == "--debug") {
        println!("{}", file_contents);
        println!("N = {:?}", N);
        println!("B = {:?}", B);
        println!("c = {}", c);
        println!("b = {}", b);
        println!("A = {}", A);
        println!("{}", "#".repeat(50));
    }

    let solve_result = if !(b.min() < 0.0) {
        // Primal-feasible
        eprintln!("Solving primal problem");
        solve_primal(A, b, c, n, m, B, N)?
    } else if !(c.max() > 0.0) {
        // Dual-feasible
        eprintln!("Solving dual problem");
        solve_dual(A, b, c, n, m, B, N)?
    } else {
        let zero = DVector::<f64>::zeros(c.len());
        eprintln!("Solving aux problem");
        match solve_dual(A.clone_owned(), b.clone_owned(), zero, n, m, B, N)? {
            Solution::Optimal(aux_solution) => {
                solve_primal(A, b, c, n, m, aux_solution.B, aux_solution.N)?
            }
            Solution::Unbounded => Solution::Infeasible,
            Solution::Infeasible => Solution::Infeasible,
        }
    };

    println!("{}", solve_result);

    Ok(())
}
