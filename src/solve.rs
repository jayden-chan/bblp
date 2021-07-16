use crate::util::{
    inv, mat_col_slice, materialize_view, round_7, vec_row_slice,
};
use nalgebra::{DMatrix, DVector};
use std::fmt;

const EPSILON: f64 = 0.000000001;

pub struct SolutionResults {
    objective_value: f64,
    variable_values: Vec<f64>,
    pub B: Vec<usize>,
    pub N: Vec<usize>,
}

pub enum Solution {
    Infeasible,
    Unbounded,
    Optimal(SolutionResults),
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Solution::Infeasible => write!(f, "infeasible"),
            Solution::Unbounded => write!(f, "unbounded"),
            Solution::Optimal(results) => {
                let x_vals = results
                    .variable_values
                    .iter()
                    .map(|v| format!("{}", round_7(*v)))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(
                    f,
                    "optimal\n{}\n{}",
                    round_7(results.objective_value),
                    x_vals
                )
            }
        }
    }
}

pub fn solve_primal(
    A: DMatrix<f64>,
    b: DVector<f64>,
    c: DVector<f64>,
    n: usize,
    m: usize,
    B: Vec<usize>,
    N: Vec<usize>,
) -> Result<Solution, String> {
    let mut B = B;
    let mut N = N;

    let mut x = DVector::<f64>::zeros(m + n);
    let x_B = mat_col_slice(&A, &B)
        .lu()
        .solve(&b)
        .ok_or_else(|| String::from("Failed to solve AB outer"))?;
    materialize_view(&mut x, &x_B, &B);

    if x_B.min() < -EPSILON {
        return Err(String::from("Initial basis is not feasible."));
    }

    let mut iterations = 0;
    loop {
        let x_B = vec_row_slice(&x, &B);
        let c_B = vec_row_slice(&c, &B);
        let c_N = vec_row_slice(&c, &N);

        let A_B = mat_col_slice(&A, &B);
        let A_N = mat_col_slice(&A, &N);

        let v = A_B
            .transpose()
            .lu()
            .solve(&c_B)
            .ok_or_else(|| String::from("Failed to solve A_B_T decomp"))?;

        let mut z = DVector::<f64>::zeros(m + n);
        let z_N = A_N.transpose() * v - c_N;
        materialize_view(&mut z, &z_N, &N);

        if !(z_N.min() < -EPSILON) {
            eprintln!("iterations = {}", iterations);

            let zeta_star = (c_B.transpose()
                * inv(A_B).expect("AB inv inner").clone_owned()
                * b.clone_owned())[0];

            return Ok(Solution::Optimal(SolutionResults {
                objective_value: zeta_star,
                variable_values: x.iter().take(n).map(|f| *f).collect(),
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
            return Ok(Solution::Unbounded);
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
            // This assignment has turned into more of an exercise in
            // floating point quirks than linear programming
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

pub fn solve_dual(
    A: DMatrix<f64>,
    b: DVector<f64>,
    c: DVector<f64>,
    n: usize,
    m: usize,
    B: Vec<usize>,
    N: Vec<usize>,
) -> Result<Solution, String> {
    let mut B = B;
    let mut N = N;

    let c_B = vec_row_slice(&c, &B);
    let c_N = vec_row_slice(&c, &N);
    let A_B = mat_col_slice(&A, &B);
    let A_N = mat_col_slice(&A, &N);

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
        let c_B = vec_row_slice(&c, &B);
        let z_B = vec_row_slice(&z, &B);
        let z_N = vec_row_slice(&z, &N);
        let A_B = mat_col_slice(&A, &B);
        let A_N = mat_col_slice(&A, &N);

        let mut x = DVector::<f64>::zeros(m + n);
        let x_B = mat_col_slice(&A, &B)
            .lu()
            .solve(&b)
            .ok_or_else(|| String::from("Failed to solve AB outer"))?;
        materialize_view(&mut x, &x_B, &B);

        if !(x_B.min() < -EPSILON) {
            eprintln!("iterations = {}", iterations);

            let zeta_star = (c_B.transpose()
                * inv(A_B).expect("AB inv inner").clone_owned()
                * b.clone_owned())[0];

            return Ok(Solution::Optimal(SolutionResults {
                objective_value: zeta_star,
                variable_values: x.iter().take(n).map(|f| *f).collect(),
                B,
                N,
            }));
        }

        let i = *B.iter().find(|idx| x[**idx] < -EPSILON).unwrap();
        let mut u = DVector::<f64>::zeros(z_B.len());
        B.iter().enumerate().for_each(|(k, b_idx)| {
            if *b_idx == i {
                u[k] = 1.0
            } else {
                u[k] = 0.0
            }
        });
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
            eprintln!("iterations = {}", iterations);
            return Ok(Solution::Unbounded);
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
