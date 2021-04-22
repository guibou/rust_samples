// Brainfuck operands
#[derive(Debug)]
pub enum Op {
    Change(i32),
    Move(i32),
    Print,
    Read,
    Loop(Vec<Op>),
}
