use na::{DMatrix, DVector, Dim, VecStorage};
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
    let mut lines = file_contents.lines();
    let c = lines.next();
    if c.is_none() {
        return Err(format!("Not enough lines in input file"));
    }

    let A: Vec<Vec<f64>> = lines
        .map(|l| {
            l.split_whitespace()
                .map(|val| val.parse::<f64>().unwrap())
                .collect()
        })
        .collect();

    let n = A.len();
    let m = A[0].len();
    let A: Vec<f64> = A.iter().flatten().map(|f| f.to_owned()).collect();
    let A = DMatrix::from_vec(n, m, A);

    let c: Vec<f64> = c
        .unwrap()
        .split_whitespace()
        .map(|val| val.parse::<f64>().unwrap())
        .collect();
    let c = DVector::from_vec(c);

    println!("{:#?}", c);
    println!("{:#?}", A);

    Ok(())
}
