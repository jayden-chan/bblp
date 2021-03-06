/*
 * Copyright © 2021 Jayden Chan. All rights reserved.
 *
 * bblp is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3
 * as published by the Free Software Foundation.
 *
 * bblp is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with bblp. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::{Matrix, Vector};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/**
 * Read a file into a string (with error handling)
 */
pub fn read_file(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => return Err(format!("couldn't open {}: {}", display, why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => Err(format!("couldn't read {}: {}", display, why)),
        Ok(_) => Ok(s),
    }
}

/**
 * Represents a parsed linear program which can be solved
 * by `solve_primal` or `solve_dual`
 */
pub struct ParsedLP {
    pub A: Matrix,
    pub b: Vector,
    pub c: Vector,
    pub n: usize,
    pub m: usize,
}

/**
 * Parse the contents of a file into the relevant matrices and vectors
 * needed to solve it with the Revised Simplex Method
 */
pub fn parse(file_contents: &str) -> Result<ParsedLP, String> {
    let mut lines = file_contents.lines().filter(|c| !c.trim().is_empty());
    let c = lines.next();

    if c.is_none() {
        return Err(String::from("Not enough lines in input file"));
    }

    let c: Vec<f64> = c
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse::<f64>().unwrap())
        .map(|val| if val == -0.0 { 0.0 } else { val })
        .collect();

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

    let A: Vec<f64> = A.into_iter().flatten().collect();
    let A = Matrix::from_row_slice(m, n, &A);

    let b = A.column(n - 1).clone_owned();
    let n = n - 1;

    // We will only insert m - 1 columnns into A since we can re-use the column that
    // was copied to `b` and no longer needed
    let mut A = A.insert_columns(n, m - 1, 0.0);

    let I = Matrix::identity(m, m);

    I.column_iter()
        .enumerate()
        .for_each(|(i, val)| A.set_column(n + i, &val));

    let c_len = c.len();
    let c = Vector::from_vec(c).insert_rows(c_len, m, 0.0);

    Ok(ParsedLP { A, b, c, n, m })
}
