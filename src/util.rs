use nalgebra::{DMatrix, DVector};

#[inline(always)]
pub fn float_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

// Construct a new Matrix comprised of the columns
// of `M` given by the indexes in `idxs`
pub fn mat_col_slice(M: &DMatrix<f64>, idxs: &Vec<usize>) -> DMatrix<f64> {
    let mut ret = DMatrix::<f64>::zeros(M.nrows(), idxs.len());
    idxs.iter()
        .enumerate()
        .for_each(|(i, idx)| ret.set_column(i, &M.column(*idx)));
    ret
}

// TODO: fix docs
// Construct a new Matrix comprised of the columns
// of `M` given by the indexes in `idxs`
pub fn vec_row_slice(M: &DVector<f64>, idxs: &Vec<usize>) -> DVector<f64> {
    let mut ret = DVector::<f64>::zeros(idxs.len());
    idxs.iter()
        .enumerate()
        .for_each(|(i, idx)| ret.set_row(i, &M.row(*idx)));
    ret
}
