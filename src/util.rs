use crate::{Matrix, Vector, EPSILON, PERTURB_AMT};

/**
 * Round `value` to `d` significant digits.
 * This function is only used for formatting the
 * results, it is not used when actually computing
 * the solution. It is necessary because Rust doesn't
 * support the %g format specifier.
 */
pub fn round_sig_figs(value: f64, d: u32) -> f64 {
    if value.abs() < f64::EPSILON {
        return 0.0;
    }

    let factor = 10_f64.powf(f64::from(d) - value.abs().log10().ceil());
    let result = (value * factor).round() / factor;
    if result.abs() < 1e-10 {
        0.0
    } else {
        result
    }
}

/**
 * Select the entering variable from the index set `N`
 * based on Dantzig's largest-coefficient rule.
 */
pub fn select_entering(N: &[usize], coefs: &Vector) -> Option<(usize, usize)> {
    let (_, j, j_idx) = N.iter().enumerate().fold(
        (-EPSILON, 0, usize::MAX),
        |acc, (idx, N_val)| {
            let item = coefs[*N_val];
            if item < acc.0 {
                (item, *N_val, idx)
            } else {
                acc
            }
        },
    );

    // if j_idx still equals usize::MAX it means there are no
    // negative coefficients so there cannot be a pivot.
    if j_idx == usize::MAX {
        None
    } else {
        Some((j, j_idx))
    }
}

pub fn select_leaving(
    B: &[usize],
    vars: &Vector,
    delta_vars: &Vector,
) -> (f64, usize, usize) {
    B.iter()
        .enumerate()
        .filter_map(|(idx, B_val)| {
            let vars_i = vars[*B_val];
            let delta_vars_i = delta_vars[*B_val];

            if delta_vars_i > EPSILON {
                Some((vars_i / delta_vars_i, *B_val, idx))
            } else {
                None
            }
        })
        // I sure hope the partial_cmp unwrap is safe here...
        // This project has turned into more of an exercise in
        // floating point safety than linear programming!
        .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
        .unwrap()
}

/**
 * Perturb the vector b based on the method described here:
 * <https://people.math.carleton.ca/~kcheung/math/notes/MATH5801/1/01_perturb.html>
 *
 * This prevents cycling and allows the use of the largest-coefficient
 * rule for all pivots.
 */
pub fn perturb(A: &Matrix, B: &[usize], b: &Vector) -> Vector {
    let A_B = col_slice(A, B);
    let m = A.nrows();
    let e = Vector::from_iterator(
        m,
        (0..m).map(|idx| PERTURB_AMT.powi(idx as i32 + 1)),
    );
    b + A_B * e
}

/********************************************************/
/*                  Index set support                   */
/*                                                      */
/* The linear algebra library I'm using doesn't support */
/* index sets so there are some extra functions needed  */
/* for doing that.                                      */
/********************************************************/

/**
 * Construct a new matrix consisting of the columns from `M`
 * given by the indices from `idxs`
 */
pub fn col_slice(M: &Matrix, idxs: &[usize]) -> Matrix {
    let mut ret = Matrix::zeros(M.nrows(), idxs.len());
    idxs.iter()
        .enumerate()
        .for_each(|(i, idx)| ret.set_column(i, &M.column(*idx)));
    ret
}

/**
 * Construct a new vector consisting of the elements from `V`
 * given by the indices from `idxs`
 */
pub fn row_slice(V: &Vector, idxs: &[usize]) -> Vector {
    let mut ret = Vector::zeros(idxs.len());
    idxs.iter()
        .enumerate()
        .for_each(|(i, idx)| ret.set_row(i, &V.row(*idx)));
    ret
}

/**
 * Copy the elements from `view` into `main` according
 * to the indices given by `idxs`.
 */
pub fn materialize_view(main: &mut Vector, view: &Vector, idxs: &[usize]) {
    idxs.iter()
        .enumerate()
        .for_each(|(e, i)| main[*i] = view[e]);
}
