use crate::{Matrix, Vector, PERTURB_AMT};

/**
 * Round `value` to `d` significant digits.
 * This function is only used on the resulting
 * objective value and points when there is an
 * optimal solution, it's not used in the `primal`
 * or `dual` functions for computing the solution.
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
