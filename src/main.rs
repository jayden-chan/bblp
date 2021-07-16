#![allow(non_snake_case)]

extern crate nalgebra as na;

mod parse;
mod solve;
mod util;

fn main() -> Result<(), String> {
    let (args, flags): (Vec<String>, Vec<String>) =
        std::env::args().partition(|a| !a.starts_with("--"));

    let stdin = String::from("/dev/stdin");
    let path = args.get(1).unwrap_or(&stdin);

    let file_contents = parse::read_file(path)?;
    let parsed = parse::parse(&file_contents)?;

    if parsed.b.min() < 0.0 {
        return Err(String::from("Can't solve this LP yet"));
    }

    let N: Vec<usize> = (0..parsed.n).collect();
    let B: Vec<usize> = (parsed.n..parsed.n + parsed.m).collect();

    if flags.iter().any(|f| f == "--debug") {
        println!("{}", file_contents);
        println!("N = {:?}", N);
        println!("B = {:?}", B);
        println!("c = {}", parsed.c);
        println!("b = {}", parsed.b);
        println!("A = {}", parsed.A);
        println!("{}", "#".repeat(50));
    }

    let solve_result = solve::solve_primal(parsed, B, N)?;
    println!("{}", solve_result);

    Ok(())
}
