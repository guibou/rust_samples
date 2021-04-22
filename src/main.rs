mod ast;
mod eval;
mod parse;

use eval::eval_bf;
use parse::parse_bf;

fn main() {
    let s = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let v = s.chars().collect();

    match parse_bf(&v) {
        Ok(ops) => {
            println!("Program: {:?}", ops);
            match eval_bf(&ops) {
                Ok(mem) => println!("Memory: {:?}", mem),
                Err(_e) => println!("Error in eval"),
            }
        }
        Err(_e) => println!("Error in parse"),
    }
}
