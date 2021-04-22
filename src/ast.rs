// Brainfuck operands
#[derive(Debug)]
pub enum Op {
    Inc,
    Dec,
    Forward,
    Backward,
    Print,
    Read,
    Loop(Vec<Op>),
}
