use nalgebra::{DMatrix, DVector};

pub fn round_sig_figs(value: f64, digits: u32) -> f64 {
    if value.abs() < f64::EPSILON {
        return 0.0;
    }

    let factor = 10f64.powf(digits as f64 - value.abs().log10().ceil());
    let result = (value * factor).round() / factor;
    return if result.abs() < 0.0000000001 {
        0.0
    } else {
        result
    };
}

/**
 * Construct a new Matrix comprised of the columns
 * of `M` given by the indexes in `idxs`
 */
pub fn mat_col_slice(M: &DMatrix<f64>, idxs: &Vec<usize>) -> DMatrix<f64> {
    let mut ret = DMatrix::<f64>::zeros(M.nrows(), idxs.len());
    idxs.iter()
        .enumerate()
        .for_each(|(i, idx)| ret.set_column(i, &M.column(*idx)));
    ret
}

/**
 * TODO: fix docs
 * Construct a new Matrix comprised of the columns
 * of `M` given by the indexes in `idxs`
 */
pub fn vec_row_slice(M: &DVector<f64>, idxs: &Vec<usize>) -> DVector<f64> {
    let mut ret = DVector::<f64>::zeros(idxs.len());
    idxs.iter()
        .enumerate()
        .for_each(|(i, idx)| ret.set_row(i, &M.row(*idx)));
    ret
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
