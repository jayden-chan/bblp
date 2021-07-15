use na::{DMatrix, DVector};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(path: &str) -> Result<String, String> {
    // Create a path to the desired file
    let path = Path::new(path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => return Err(format!("couldn't open {}: {}", display, why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => Err(format!("couldn't read {}: {}", display, why)),
        Ok(_) => Ok(s),
    }
}

pub struct ParsedLP {
    pub A: DMatrix<f64>,
    pub b: DVector<f64>,
    pub c: DVector<f64>,
    pub n: usize,
    pub m: usize,
}

pub fn parse(file_contents: &str) -> Result<ParsedLP, String> {
    let mut lines = file_contents.lines();
    let c = lines.next();

    if c.is_none() {
        return Err(String::from("Not enough lines in input file"));
    }

    // Read value of top row into a vector
    let c: Vec<f64> = c
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse::<f64>().unwrap())
        .map(|val| if val == -0.0 { 0.0 } else { val })
        .collect();

    // Read the rest of the lines
    let A: Vec<Vec<f64>> = lines
        .map(|l| {
            l.split_whitespace()
                .map(|val| val.parse::<f64>().unwrap())
                .map(|val| if val == -0.0 { 0.0 } else { val })
                .collect()
        })
        .collect();

    let n = A[0].len();
    let m = A.len();

    if n == 0 || m == 0 {
        return Err(String::from("Not enough rows/cols for matrix A"));
    }

    // Convert A from 2D vector to 1D vector with row-major storage
    let A: Vec<f64> = A.iter().flatten().map(f64::to_owned).collect();
    let A = DMatrix::from_row_slice(m, n, &A);

    // Grab the b column off of A
    let b = A.column(n - 1).clone_owned();
    let n = n - 1;

    // Compute the mxm identity matrix to append to A
    let I = DMatrix::<f64>::identity(m, m);

    // We will only insert m - 1 columnns into A since we can re-use the column that
    // was copied to `b` and no longer needed
    let mut A = A.insert_columns(n, m - 1, 0.0);

    // Append I columns to A
    I.column_iter()
        .enumerate()
        .for_each(|(i, val)| A.set_column(n + i, &val));

    let c_len = c.len();
    let c = DVector::from_vec(c).insert_rows(c_len, m + n - c_len, 0.0);

    Ok(ParsedLP { A, b, c, n, m })
}
