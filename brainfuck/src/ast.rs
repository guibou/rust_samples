use std::collections::HashMap;

// Brainfuck operands
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Op {
    // Change({o:d}) memory[O + o] += d
    Change(HashMap<i32, i32>),
    // Change({o:d}) memory[O + o] = d
    Set(HashMap<i32, i32>),
    // O += i
    Move(i32),
    // Print value at memory[O]
    Print(i32),
    // Print value to memory[O]
    Read(i32),
    // Loop until memory[O] == 0
    Loop(Vec<Op>),
    // AddMul(offsetResult, offsetA, valueB)
    // memory[O + offsetResult] += memory[O + offsetOerandA] * valueB
    AddMul(i32, i32, i32),
}
