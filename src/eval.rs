use crate::ast::Op;

// Evaluate a brainfuck program and returns the memory state
pub fn eval_bf(ops: &Vec<Op>) -> Result<Vec<i32>, String> {
    let mut memory: Vec<i32> = Vec::new();
    let mut offset: i32 = 0;
    match eval_bf2(&mut memory, &mut offset, ops) {
        Ok(()) => Ok(memory),
        Err(e) => Err(e),
    }
}

// Internal: evaluate a brainfuck program with current memory and memory offset
fn eval_bf2(memory: &mut Vec<i32>, offset: &mut i32, ops: &Vec<Op>) -> Result<(), String> {
    for op in ops {
        match op {
            Op::Set(m) => {
                for (doffset, val) in m {
                    *(on_memory(memory, *offset + *doffset)?) = *val;
                }
            }
            Op::Change(m) => {
                for (doffset, val) in m {
                    *(on_memory(memory, *offset + *doffset)?) += *val;
                }
            }
            Op::Move(i) => *offset += i,
            Op::Print(i) => {
                let u = *(on_memory(memory, *offset + i)?);
                let c = std::char::from_u32(u as u32).unwrap_or('?');

                print!("{:}", c);
            }
            Op::Read(_i) => (),
            Op::Loop(ops2) => {
                while *(on_memory(memory, *offset)?) != 0 {
                    eval_bf2(memory, offset, &ops2)?;
                }
            }
            Op::AddMul(or, oa, b) => {
                *(on_memory(memory, *offset + *or)?) += *(on_memory(memory, *offset + *oa)?) * b;
            }
        }
    }
    Ok(())
}

// Returns a memory offset (and allocate it if needed)
fn on_memory(memory: &mut Vec<i32>, offset: i32) -> Result<&mut i32, String> {
    if offset < 0 {
        Err("Access out of bounds < 0".to_string())
    } else {
        while offset >= memory.len() as i32 {
            memory.push(0);
        }

        Ok(&mut memory[offset as usize])
    }
}
