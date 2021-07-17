use crate::util::{col_slice, materialize_view, round_sig_figs, row_slice};
use crate::EPSILON;
use nalgebra::{DMatrix, DVector};
use std::fmt;

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
 * Format the results
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
 * Primal simplex solve routine as described on
 * slide 73 of lecture 14
 */
pub fn primal(
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

    let mut x = DVector::<f64>::zeros(m + n);
    let x_B = col_slice(A, &B)
        .lu()
        .solve(b)
        .ok_or_else(|| String::from("Failed to solve AB outer"))?;
    materialize_view(&mut x, &x_B, &B);

    if x_B.min() < -EPSILON {
        return Err(String::from("Initial basis is not feasible."));
    }

    let mut iterations = 0;
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

        let mut z = DVector::<f64>::zeros(m + n);
        let z_N = A_N.transpose() * v - c_N;
        materialize_view(&mut z, &z_N, &N);

        if !(z_N.min() < -EPSILON) {
            let objective_value = (c_B.transpose() * x_B)[0];
            return Ok(SolveResult::Optimal(Solution {
                variable_values: x.iter().take(n).copied().collect(),
                objective_value,
                B,
                N,
            }));
        }

        let j = *N.iter().find(|idx| z[**idx] < -EPSILON).unwrap();
        let mut delta_x = DVector::<f64>::zeros(m + n);
        let delta_x_B = A_B
            .lu()
            .solve(&A.column(j))
            .ok_or_else(|| String::from("Failed to solve for delta_x_B"))?;

        materialize_view(&mut delta_x, &delta_x_B, &B);

        if !(delta_x.max() > EPSILON) {
            return Ok(SolveResult::Unbounded);
        }

        let (t, i) = B
            .iter()
            .filter_map(|idx| {
                let x_i = x[*idx];
                let delta_x_i = delta_x[*idx];

                if delta_x_i > EPSILON {
                    Some((x_i / delta_x_i, *idx))
                } else {
                    None
                }
            })
            // I sure hope the partial_cmp unwrap is safe here...
            // This project has turned into more of an exercise in
            // floating point safety than linear programming!
            .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
            .unwrap();

        materialize_view(&mut x, &(x_B.clone_owned() - t * delta_x_B), &B);
        x[j] = t;

        let B_replace_idx = B.iter().position(|idx| *idx == i).unwrap();
        let N_replace_idx = N.iter().position(|idx| *idx == j).unwrap();
        B[B_replace_idx] = j;
        N[N_replace_idx] = i;
        iterations += 1;
    }
}

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
