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
 * Dual simplex solve routine as described on
 * slide 97 of lecture 14
 */
pub fn dual(
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

    if z_N.min() < -EPSILON {
        return Err(String::from("Initial basis is not feasible."));
    }

    let mut pivots = 0;
    loop {
        let z_B = row_slice(&z, &B);
        let z_N = row_slice(&z, &N);
        let A_B = col_slice(A, &B);
        let A_N = col_slice(A, &N);
        let c_B = row_slice(c, &B);

        let mut x = Vector::zeros(m + n);
        let x_B = col_slice(A, &B)
            .lu()
            .solve(&b)
            .ok_or_else(|| String::from("Failed to solve for x_B"))?;
        write_view(&mut x, &x_B, &B);

        let (i, i_idx) = match select_entering(&B, &x) {
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
            Some((i, i_idx)) => (i, i_idx),
        };

        // Compute u as described on slide 91
        let mut u = Vector::zeros(z_B.len());
        u[i_idx] = 1.0;
        let u = u;

        let mut delta_z = Vector::zeros(m + n);
        let v = A_B
            .transpose()
            .lu()
            .solve(&u)
            .ok_or_else(|| String::from("Failed to solve for v"))?;

        let delta_z_N = -(A_N.transpose() * v);
        write_view(&mut delta_z, &delta_z_N, &N);

        if !(delta_z.max() > EPSILON) {
            return Ok(SolveResult::Infeasible);
        }

        let (s, j, j_idx) = select_leaving(&N, &z, &delta_z);
        let z_N = z_N.clone_owned() - s * delta_z_N;
        write_view(&mut z, &z_N, &N);
        z[i] = s;

        B[i_idx] = j;
        N[j_idx] = i;
        pivots += 1;
    }
}
