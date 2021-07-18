use crate::solve::{Solution, SolveResult};
use crate::util::{col_slice, materialize_view, perturb, row_slice};
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
) -> Result<SolveResult, String> {
    let mut B = B;
    let mut N = N;
    let b = perturb(A, &B, &b);

    let n = N.len();
    let m = B.len();

    let mut x = Vector::zeros(m + n);
    let x_B = col_slice(A, &B)
        .lu()
        .solve(&b)
        .ok_or_else(|| String::from("Failed to solve AB outer"))?;
    materialize_view(&mut x, &x_B, &B);

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

        let v = A_B
            .transpose()
            .lu()
            .solve(&c_B)
            .ok_or_else(|| String::from("Failed to solve A_B_T decomp"))?;

        let mut z = Vector::zeros(m + n);
        let z_N = A_N.transpose() * v - c_N;
        materialize_view(&mut z, &z_N, &N);

        if !(z_N.min() < -EPSILON) {
            let objective_value = (c_B.transpose() * x_B)[0];
            return Ok(SolveResult::Optimal(Solution {
                variable_values: x.iter().take(n).copied().collect(),
                objective_value,
                pivots,
                B,
                N,
            }));
        }

        // let j = *N.iter().find(|idx| z[**idx] < -EPSILON).unwrap();
        let (_, j, j_idx) =
            N.iter()
                .enumerate()
                .fold((-EPSILON, 0, 0), |acc, (idx, N_val)| {
                    let item = z[*N_val];
                    if item < acc.0 {
                        (item, *N_val, idx)
                    } else {
                        acc
                    }
                });

        let mut delta_x = Vector::zeros(m + n);
        let delta_x_B = A_B
            .lu()
            .solve(&A.column(j))
            .ok_or_else(|| String::from("Failed to solve for delta_x_B"))?;

        materialize_view(&mut delta_x, &delta_x_B, &B);

        if !(delta_x.max() > EPSILON) {
            return Ok(SolveResult::Unbounded);
        }

        let (t, i, i_idx) = B
            .iter()
            .enumerate()
            .filter_map(|(idx, B_val)| {
                let x_i = x[*B_val];
                let delta_x_i = delta_x[*B_val];

                if delta_x_i > EPSILON {
                    Some((x_i / delta_x_i, *B_val, idx))
                } else {
                    None
                }
            })
            // I sure hope the partial_cmp unwrap is safe here...
            // This project has turned into more of an exercise in
            // floating point safety than linear programming!
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap();

        materialize_view(&mut x, &(x_B.clone_owned() - t * delta_x_B), &B);
        x[j] = t;

        B[i_idx] = j;
        N[j_idx] = i;
        pivots += 1;
    }
}
