use std::fmt;

use nalgebra::DVector;

use crate::{
    parse::ParsedLP,
    util::{inv, mat_col_slice, materialize_view, round_7, vec_row_slice},
};

const EPSILON: f64 = 0.00000001;

pub enum Solution {
    Infeasible,
    Unbounded,
    Optimal(f64, Vec<f64>),
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Solution::Infeasible => write!(f, "infeasible"),
            Solution::Unbounded => write!(f, "unbounded"),
            Solution::Optimal(obj_val, point) => {
                let points: Vec<String> =
                    point.iter().map(|v| format!("{}", round_7(*v))).collect();
                let points = points.join(" ");
                write!(f, "optimal\n{}\n{}", round_7(*obj_val), points)
            }
        }
    }
}

pub fn solve_primal(
    parsed_lp: ParsedLP,
    B: Vec<usize>,
    N: Vec<usize>,
) -> Result<Solution, String> {
    let A = parsed_lp.A;
    let b = parsed_lp.b;
    let c = parsed_lp.c;
    let n = parsed_lp.n;
    let m = parsed_lp.m;
    let mut B = B;
    let mut N = N;

    let mut x = DVector::<f64>::zeros(m + n);
    let x_B = mat_col_slice(&A, &B)
        .lu()
        .solve(&b)
        .ok_or_else(|| String::from("Failed to solve AB outer"))?;
    materialize_view(&mut x, &x_B, &B);

    if x_B.min() < 0.0 {
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
            println!("iterations = {}", iterations);
            let zeta_star = (c_B.transpose()
                * inv(A_B).expect("AB inv inner").clone_owned()
                * b.clone_owned())[0];
            return Ok(Solution::Optimal(
                zeta_star,
                x.iter().take(n).map(|f| *f).collect(),
            ));
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
            .min_by(|(x, _), (y, _)| x.partial_cmp(y).unwrap())
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
