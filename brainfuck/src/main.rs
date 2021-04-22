mod ast;
mod eval;
mod optimise;
mod parse;

use eval::eval_bf;
use optimise::optimise;
use parse::parse_bf;
use std::io::{Error, ErrorKind};

fn main() -> std::io::Result<()> {
    let filename = std::env::args().nth(1);

    let hanoi = std::fs::read_to_string(filename.unwrap_or("programs/hello.bf".to_string()))?;
    let v = hanoi.chars().collect();

    let bf = parse_bf(&v).or_else(|x| Err(Error::new(ErrorKind::Other, x)))?;
    let optimised = optimise(bf);
    let memory = eval_bf(&optimised).or_else(|x| Err(Error::new(ErrorKind::Other, x)))?;

    println!("{:?}", memory);

    Ok(())
}
