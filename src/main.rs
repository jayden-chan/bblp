#![allow(non_snake_case)]
extern crate nalgebra as na;

mod parse;
mod util;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let stdin_path = String::from("/dev/stdin");
    let path = args.get(1).unwrap_or(&stdin_path);

    let contents = parse::read_file(path)?;
    match parse::parse(&contents) {
        Ok(_) => println!("parse ok"),
        Err(e) => eprintln!("{}", e),
    }
    Ok(())
}
