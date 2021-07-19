/*
 * Copyright © 2021 Jayden Chan. All rights reserved.
 *
 * bblp is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3
 * as published by the Free Software Foundation.
 *
 * bblp is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with bblp. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::solve::{Solution, SolveResult};
use crate::util::{
    col_view, perturb, row_view, select_entering, select_leaving, write_view,
};
use crate::{Matrix, Vector, EPSILON};

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

    // Perturb the `b` vector if that setting is enabled. I've been
    // doing a bit more reading on perturbation since I originally implemented
    // it and now I'm not actually sure if this has any effect for the dual simplex
    // algorithm. The source I was going off of only described perturbation for the
    // primal method, and I just applied that here too. But I think for dual simplex
    // a different perturbation method is needed.
    let b = if no_perturb {
        b.clone_owned()
    } else {
        perturb(A, &B, b)
    };

    let c_B = row_view(c, &B);
    let c_N = row_view(c, &N);
    let A_B = col_view(A, &B);
    let A_N = col_view(A, &N);

    // Compute z by solving A_B^T * v = c_B then setting z_N = A_N^T * v - c_N
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
        let z_B = row_view(&z, &B);
        let z_N = row_view(&z, &N);
        let A_B = col_view(A, &B);
        let A_N = col_view(A, &N);
        let c_B = row_view(c, &B);

        // Compute x_B by solving A_B * x_B = b
        let mut x = Vector::zeros(m + n);
        let x_B = col_view(A, &B)
            .lu()
            .solve(&b)
            .ok_or_else(|| String::from("Failed to solve for x_B"))?;
        write_view(&mut x, &x_B, &B);

        // Select our entering variable using the largest coefficient rule.
        // If there is no suitable entering variable it means we have
        // reached an optimal solution.
        let (i, i_idx) = match select_entering(&B, &x) {
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

        let mut u = Vector::zeros(z_B.len());
        u[i_idx] = 1.0;
        let u = u;

        // Compute delta_z_N by solving A_B^T * v = u and setting z_N = -A_N^T * v
        let mut delta_z = Vector::zeros(m + n);
        let v = A_B
            .transpose()
            .lu()
            .solve(&u)
            .ok_or_else(|| String::from("Failed to solve for v"))?;

        let delta_z_N = -(A_N.transpose() * v);
        write_view(&mut delta_z, &delta_z_N, &N);

        // Select our leaving variable. If there is no leaving
        // variable the problem is unbounded (primal infeasible)
        let (s, j, j_idx) = match select_leaving(&N, &z, &delta_z) {
            None => return Ok(SolveResult::Infeasible),
            Some(p) => p,
        };

        let z_N = z_N.clone_owned() - s * delta_z_N;
        write_view(&mut z, &z_N, &N);

        z[i] = s;
        B[i_idx] = j;
        N[j_idx] = i;
        pivots += 1;
    }
}
