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

// Take the inverse of matrix M and convert the Option<M>
// result to Result<M, String>
#[inline(always)]
pub fn inv(M: DMatrix<f64>) -> Result<DMatrix<f64>, String> {
    M.try_inverse()
        .ok_or_else(|| String::from("Matrix isn't inverible"))
}

pub fn materialize_view(
    main: &mut DVector<f64>,
    view: &DVector<f64>,
    idxs: &Vec<usize>,
) {
    idxs.iter()
        .enumerate()
        .for_each(|(e, i)| main[*i] = view[e]);
}
