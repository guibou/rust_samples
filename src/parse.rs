use crate::ast::Op;

// Parse a vector of char as a brainfuck list of operands
pub fn parse_bf(s: &Vec<char>) -> Result<Vec<Op>, String> {
    let mut stack = Vec::new();
    let mut current = Vec::new();

    for c in s {
        match c {
            '[' => {
                stack.push(current);
                current = Vec::new();
            }
            ']' => {
                let op = Op::Loop(current);
                match stack.pop() {
                    Some(c) => {
                        current = c;
                        current.push(op);
                    }
                    None => {
                        return Err("error in stack".to_string());
                    }
                }
            }
            _ => {
                let op = match c {
                    '+' => Op::Inc,
                    '-' => Op::Dec,
                    '>' => Op::Forward,
                    '<' => Op::Backward,
                    '.' => Op::Print,
                    ',' => Op::Read,
                    _ => return Err("Wrong char".to_string()),
                };
                current.push(op);
            }
        }
    }
    if stack.len() > 0 {
        Err("Unclosed loops".to_string())
    } else {
        Ok(current)
    }
}
