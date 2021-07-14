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

pub fn parse(file_contents: &str) -> Result<(), String> {
    println!("{}", file_contents);
    let mut lines = file_contents.lines();
    let c = lines.next();

    if c.is_none() {
        return Err(String::from("Not enough lines in input file"));
    }

    // Compute c from the first line of text
    let c: Vec<f64> = c
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse::<f64>().unwrap())
        .collect();
    let c = DVector::from_vec(c);

    let A: Vec<Vec<f64>> = lines
        .map(|l| {
            l.split_whitespace()
                .map(|val| val.parse::<f64>().unwrap())
                .collect()
        })
        .collect();

    let n = A.len();
    let m = A[0].len();

    if n == 0 || m == 0 {
        return Err(String::from("Not enough rows/cols for matrix A"));
    }

    // Convert A from 2D vector to 1D vector in row-major
    let A: Vec<f64> = A.iter().flatten().map(f64::to_owned).collect();
    let A = DMatrix::from_row_slice(n, m, &A);

    // Grab the w column off of A
    let w = A.column(m - 1).clone_owned();
    let m = m - 1;

    println!("{}x{}", n, m);

    // Compute the mxm identity matrix to append to A
    let I = DMatrix::<f64>::identity(m, m);
    let mut A = A.insert_columns(m, m - 1, 0.0);

    I.column_iter()
        .enumerate()
        .for_each(|(i, val)| A.set_column(m + i, &val));

    println!("c = {:#?}", c);
    println!("w = {:#?}", w);
    println!("A = {:#?}", A);

    Ok(())
}
