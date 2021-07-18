use crate::solve::{Solution, SolveResult};
use crate::util::{col_slice, materialize_view, perturb, row_slice};
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
) -> Result<SolveResult, String> {
    let mut B = B;
    let mut N = N;
    let b = perturb(A, &B, b);

    let n = N.len();
    let m = B.len();

    let c_B = row_slice(c, &B);
    let c_N = row_slice(c, &N);
    let A_B = col_slice(A, &B);
    let A_N = col_slice(A, &N);

    let mut z = Vector::zeros(m + n);
    let v = A_B
        .transpose()
        .lu()
        .solve(&c_B)
        .ok_or_else(|| String::from("Failed to solve A_B_T decomp"))?;
    let z_N = A_N.transpose() * v - c_N;
    materialize_view(&mut z, &z_N, &N);

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
            .ok_or_else(|| String::from("Failed to solve AB outer"))?;
        materialize_view(&mut x, &x_B, &B);

        if !(x_B.min() < -EPSILON) {
            let objective_value = (c_B.transpose() * x_B)[0];
            return Ok(SolveResult::Optimal(Solution {
                variable_values: x.iter().take(n).copied().collect(),
                objective_value,
                pivots,
                B,
                N,
            }));
        }

        // let i_idx = B.iter().position(|idx| x[*idx] < -EPSILON).unwrap();
        let (_, i, i_idx) =
            B.iter()
                .enumerate()
                .fold((-EPSILON, 0, 0), |acc, (idx, B_val)| {
                    let item = x[*B_val];
                    if item < acc.0 {
                        (item, *B_val, idx)
                    } else {
                        acc
                    }
                });

        // let i = B[i_idx];
        let mut u = Vector::zeros(z_B.len());
        u[i_idx] = 1.0;
        let u = u;

        let mut delta_z = Vector::zeros(m + n);
        let v = A_B
            .transpose()
            .lu()
            .solve(&u)
            .expect("Failed to solve for delta_z_N");
        let delta_z_N = -(A_N.transpose() * v);

        materialize_view(&mut delta_z, &delta_z_N, &N);

        if !(delta_z.max() > EPSILON) {
            eprintln!("{} pivots", pivots);
            return Ok(SolveResult::Infeasible);
        }

        let (s, j, j_idx) = N
            .iter()
            .enumerate()
            .filter_map(|(idx, N_val)| {
                let z_j = z[*N_val];
                let delta_z_j = delta_z[*N_val];

                if delta_z_j > EPSILON {
                    Some((z_j / delta_z_j, *N_val, idx))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .unwrap();

        let sdzn = s * delta_z_N;
        materialize_view(&mut z, &(z_N.clone_owned() - sdzn), &N);
        z[i] = s;

        B[i_idx] = j;
        N[j_idx] = i;
        pivots += 1;
    }
}
