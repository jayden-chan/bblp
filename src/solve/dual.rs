use crate::solve::{Solution, SolveResult};
use crate::util::{col_slice, materialize_view, row_slice};
use crate::EPSILON;
use nalgebra::{DMatrix, DVector};

/**
 * Dual simplex solve routine as described on
 * slide 97 of lecture 14
 */
pub fn dual(
    A: &DMatrix<f64>,
    b: &DVector<f64>,
    c: &DVector<f64>,
    B: Vec<usize>,
    N: Vec<usize>,
) -> Result<SolveResult, String> {
    let mut B = B;
    let mut N = N;

    let n = N.len();
    let m = B.len();

    let c_B = row_slice(c, &B);
    let c_N = row_slice(c, &N);
    let A_B = col_slice(A, &B);
    let A_N = col_slice(A, &N);

    let mut z = DVector::<f64>::zeros(m + n);
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

    let mut iterations = 0;
    loop {
        let z_B = row_slice(&z, &B);
        let z_N = row_slice(&z, &N);
        let A_B = col_slice(A, &B);
        let A_N = col_slice(A, &N);
        let c_B = row_slice(c, &B);

        let mut x = DVector::<f64>::zeros(m + n);
        let x_B = col_slice(A, &B)
            .lu()
            .solve(b)
            .ok_or_else(|| String::from("Failed to solve AB outer"))?;
        materialize_view(&mut x, &x_B, &B);

        if !(x_B.min() < -EPSILON) {
            let objective_value = (c_B.transpose() * x_B)[0];
            return Ok(SolveResult::Optimal(Solution {
                variable_values: x.iter().take(n).copied().collect(),
                objective_value,
                B,
                N,
            }));
        }

        let i_idx = B.iter().position(|idx| x[*idx] < -EPSILON).unwrap();
        let i = B[i_idx];
        let mut u = DVector::<f64>::zeros(z_B.len());
        u[i_idx] = 1.0;
        let u = u;

        let mut delta_z = DVector::<f64>::zeros(m + n);
        let v = A_B
            .transpose()
            .lu()
            .solve(&u)
            .expect("Failed to solve for delta_z_N");
        let delta_z_N = -(A_N.transpose() * v);

        materialize_view(&mut delta_z, &delta_z_N, &N);

        if !(delta_z.max() > EPSILON) {
            return Ok(SolveResult::Infeasible);
        }

        let (s, j) = N
            .iter()
            .filter_map(|idx| {
                let z_j = z[*idx];
                let delta_z_j = delta_z[*idx];

                if delta_z_j > EPSILON {
                    Some((z_j / delta_z_j, *idx))
                } else {
                    None
                }
            })
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
            .unwrap();

        let sdzn = s * delta_z_N;
        materialize_view(&mut z, &(z_N.clone_owned() - sdzn), &N);
        z[i] = s;

        let B_replace_idx = B.iter().position(|idx| *idx == i).unwrap();
        let N_replace_idx = N.iter().position(|idx| *idx == j).unwrap();
        B[B_replace_idx] = j;
        N[N_replace_idx] = i;
        iterations += 1;
    }
}
