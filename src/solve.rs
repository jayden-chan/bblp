use std::fmt;

use nalgebra::DVector;

use crate::{
    parse::ParsedLP,
    util::{mat_col_slice, vec_row_slice},
};

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
                write!(f, "optimal\n{:.7}\n{:?}", obj_val, point)
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

    let mut A_B_inverse = mat_col_slice(&A, &B).try_inverse().unwrap();
    let mut x = DVector::<f64>::zeros(m + n);
    let x_B = A_B_inverse.clone_owned() * b.clone_owned();
    let x_N = DVector::<f64>::zeros(n);

    B.iter().enumerate().for_each(|(e, i)| x[*i] = x_B[e]);
    N.iter().enumerate().for_each(|(e, i)| x[*i] = x_N[e]);

    if x_B.min() < 0.0 {
        return Err(String::from("Initial basis is not feasible."));
    }

    // print!("A_B_inverse = {}", A_B_inverse);
    // print!("x_B = {}", x_B);
    // print!("x_N = {}", x_N);
    // print!("x = {}", x);

    let mut iterations = 0;
    loop {
        let c_B = vec_row_slice(&c, &B);
        let c_N = vec_row_slice(&c, &N);
        A_B_inverse = mat_col_slice(&A, &B).try_inverse().unwrap();
        let A_N = mat_col_slice(&A, &N);

        let mut z = DVector::<f64>::zeros(m + n);
        let z_B = DVector::<f64>::zeros(m);
        B.iter().enumerate().for_each(|(e, i)| z[*i] = z_B[e]);
        let z_N = (A_B_inverse.clone_owned() * A_N).transpose()
            * c_B.clone_owned()
            - c_N;
        N.iter().enumerate().for_each(|(e, i)| z[*i] = z_N[e]);

        // print!("z_B = {}", z_B);
        // print!("z_N = {}", z_N);
        // print!("z = {}", z);

        if !(z_N.min() < 0.0) {
            let zeta_star = (c_B.transpose() * A_B_inverse * b)[0];
            return Ok(Solution::Optimal(
                zeta_star,
                x.iter().map(|f| *f).collect(),
            ));
        }

        let j = *N.iter().find(|idx| z[**idx] < 0.0).unwrap();
        let mut delta_x = DVector::<f64>::zeros(m + n);
        let delta_x_B = A_B_inverse * A.column(j);
        let delta_x_N = DVector::<f64>::zeros(n);
        B.iter()
            .enumerate()
            .for_each(|(e, i)| delta_x[*i] = delta_x_B[e]);
        N.iter()
            .enumerate()
            .for_each(|(e, i)| delta_x[*i] = delta_x_N[e]);

        if !(delta_x.max() > 0.0) {
            return Ok(Solution::Unbounded);
        }

        let (t, i) = B
            .iter()
            .filter_map(|idx| {
                let x_i = x[*idx];
                let delta_x_i = delta_x[*idx];

                if delta_x_i > 0.0 {
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

        // println!("t = {}", t);
        // println!("delta_x_B = {}", delta_x_B);
        let x_B = x_B.clone_owned() - t * delta_x_B;
        B.iter().enumerate().for_each(|(e, i)| x[*i] = x_B[e]);
        x[j] = t;

        let B_replace_idx = B.iter().position(|idx| *idx == i).unwrap();
        let N_replace_idx = N.iter().position(|idx| *idx == j).unwrap();
        B[B_replace_idx] = j;
        N[N_replace_idx] = i;
        iterations += 1;
    }
}
