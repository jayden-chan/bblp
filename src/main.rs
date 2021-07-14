#![allow(non_snake_case)]
extern crate nalgebra as na;

mod parse;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(String::from(
            "Not enough arguments provided. You must specify a file to read.",
        ));
    }

    let contents = parse::read_file(&args[1])?;
    match parse::parse(&contents) {
        Ok(_) => println!("parse ok"),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
