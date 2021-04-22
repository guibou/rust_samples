// Brainfuck operands
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Op {
    // Change(d, o) will add `d` at a distance `o` in memory.
    Change(i32, i32),
    Move(i32),
    Print(i32),
    Read(i32),
    Loop(Vec<Op>),
}
