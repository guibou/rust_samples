mod ast;
mod eval;
mod parse;
mod optimise;

use eval::eval_bf;
use parse::parse_bf;
use optimise::optimise;

fn main() {
    let s = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let v = s.chars().collect();

    match parse_bf(&v) {
        Ok(ops) => {
            println!("Program: {:?}", ops);
            let ops_optimised = optimise(ops);
            println!("\nOptimised: {:?}", ops_optimised);
            match eval_bf(&ops_optimised) {
                Ok(mem) => println!("Memory: {:?}", mem),
                Err(_e) => println!("Error in eval"),
            }
        }
        Err(_e) => println!("Error in parse"),
    }
}
