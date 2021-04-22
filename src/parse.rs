use crate::ast::Op;
use std::collections::HashMap;

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
                    '+' => Some(new_change(1)),
                    '-' => Some(new_change(-1)),
                    '>' => Some(Op::Move(1)),
                    '<' => Some(Op::Move(-1)),
                    '.' => Some(Op::Print(0)),
                    ',' => Some(Op::Read(0)),
                    _ => None,
                };
                match op {
                    Some(opp) => current.push(opp),
                    None => (),
                }
            }
        }
    }
    if stack.len() > 0 {
        Err("Unclosed loops".to_string())
    } else {
        Ok(current)
    }
}

fn new_change(i: i32) -> Op {
    let mut map = HashMap::new();
    map.insert(0, i);

    Op::Change(map)
}
