#![allow(non_snake_case)]
extern crate nalgebra as na;

mod parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Not enough arguments provided. You must specify a file to read."
        );
        return;
    }

    let contents = parse::read_file(&args[1]);
    match contents {
        Ok(s) => match parse::parse(&s) {
            Ok(_) => println!("parse oK"),
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("Failed to read input file: {}", e),
    }
}
