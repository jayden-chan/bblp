/*
 * CSC-445 Linear Program Solver
 * Jayden Chan
 * V00898517
 */

use crate::solve::{Solution, SolveResult};
use crate::util::{
    col_slice, perturb, row_slice, select_entering, select_leaving, write_view,
};
use crate::{Matrix, Vector, EPSILON};

/**
 * Primal simplex solve routine as described on
 * slide 73 of lecture 14
 */
pub fn primal(
    A: &Matrix,
    b: &Vector,
    c: &Vector,
    B: Vec<usize>,
    N: Vec<usize>,
    no_perturb: bool,
) -> Result<SolveResult, String> {
    let mut B = B;
    let mut N = N;
    let n = N.len();
    let m = B.len();

    // Perturb the `b` vector if that setting is enabled
    let b = if no_perturb {
        b.clone_owned()
    } else {
        perturb(A, &B, b)
    };

    let mut x = Vector::zeros(m + n);
    let x_B = col_slice(A, &B)
        .lu()
        .solve(&b)
        .ok_or_else(|| String::from("Failed to for x_B"))?;
    write_view(&mut x, &x_B, &B);

    if x_B.min() < -EPSILON {
        return Err(String::from("Initial basis is not feasible."));
    }

    let mut pivots = 0;
    loop {
        let x_B = row_slice(&x, &B);
        let c_B = row_slice(c, &B);
        let c_N = row_slice(c, &N);

        let A_B = col_slice(A, &B);
        let A_N = col_slice(A, &N);

        let mut z = Vector::zeros(m + n);
        let v = A_B
            .transpose()
            .lu()
            .solve(&c_B)
            .ok_or_else(|| String::from("Failed to solve for v"))?;
        let z_N = A_N.transpose() * v - c_N;
        write_view(&mut z, &z_N, &N);

        let (j, j_idx) = match select_entering(&N, &z) {
            // If there is no suitable entering variable it means we are done
            None => {
                let objective_value = (c_B.transpose() * x_B)[0];
                return Ok(SolveResult::Optimal(Solution {
                    variable_values: x.iter().take(n).copied().collect(),
                    objective_value,
                    pivots,
                    B,
                    N,
                }));
            }
            Some((j, j_idx)) => (j, j_idx),
        };

        let mut delta_x = Vector::zeros(m + n);
        let delta_x_B = A_B
            .lu()
            .solve(&A.column(j))
            .ok_or_else(|| String::from("Failed to solve for delta_x_B"))?;

        write_view(&mut delta_x, &delta_x_B, &B);

        if !(delta_x.max() > EPSILON) {
            return Ok(SolveResult::Unbounded);
        }

        let (t, i, i_idx) = select_leaving(&B, &x, &delta_x);
        write_view(&mut x, &(x_B.clone_owned() - t * delta_x_B), &B);
        x[j] = t;

        B[i_idx] = j;
        N[j_idx] = i;
        pivots += 1;
    }
}
