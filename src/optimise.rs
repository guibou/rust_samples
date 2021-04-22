use crate::ast::Op;
use std::collections::HashMap;

// Loop([Change({0: -1, X: B})]
// There is N is the loop counter
// N time, I will add B in offset X
// so this is equal to
// Set({0: 0}), Mul 0 B
// With Mul (offestoperandA, valueB, offsetResult)
// memory[offsetResult] = memory[offsetOerandA] * valueB

// Optimise single instructions
pub fn optimise_single(ops: Vec<Op>) -> Vec<Op> {
    let mut new = Vec::new();

    for o in ops {
        match o {
            Op::Move(0) => (),
            Op::Loop(l) => match l.as_slice() {
                [Op::Change(m)] => {
                    match m.get(&0) {
                        // If this loop set the current register to 0 by decrementing
                        Some(&-1) => {
                            // If there is only the set to 0, it means that it just Set the value
                            // to 0
                            if m.len() == 1 {
                                let mut set_map = HashMap::new();
                                set_map.insert(0, 0);
                                new.push(Op::Set(set_map))
                            }
                            // Otherwise, it is a Mul thing
                            else {
                                for (offset_a, val_b) in m {
                                    if *offset_a != 0 {
                                        new.push(Op::AddMul(*offset_a, 0, *val_b));
                                    }
                                }
                                let mut set_map = HashMap::new();
                                set_map.insert(0, 0);
                                new.push(Op::Set(set_map))
                            };
                        }
                        _ => new.push(Op::Loop(optimise(l))),
                    }
                }
                _ => new.push(Op::Loop(optimise(l))),
            },
            _ => new.push(o),
        }
    }

    new
}

fn move_dict(d: i32, map: &HashMap<i32, i32>) -> HashMap<i32, i32> {
    let mut new_map = HashMap::new();

    for (offset, value) in map {
        new_map.insert(offset + d, *value);
    }

    new_map
}

// optimise_peephole
pub fn optimise_peephole(ops: Vec<Op>) -> Vec<Op> {
    let mut new_ops: Vec<Op> = Vec::new();

    for op in ops {
        match new_ops.last_mut() {
            None => new_ops.push(op.clone()),
            Some(current_op) => match (&current_op, &op) {
                // Move the move to the right
                (Op::Move(m), Op::Print(doffset)) => {
                    let new_op = Op::Move(*m);
                    *current_op = Op::Print(doffset + m);
                    new_ops.push(new_op);
                }
                (Op::Move(m), Op::Read(doffset)) => {
                    let new_op = Op::Move(*m);
                    *current_op = Op::Read(doffset + m);
                    new_ops.push(new_op);
                }
                (Op::Move(m), Op::Set(map)) => {
                    let new_op = Op::Move(*m);
                    *current_op = Op::Set(move_dict(*m, map));
                    new_ops.push(new_op);
                }
                (Op::Move(m), Op::Change(map)) => {
                    let new_op = Op::Move(*m);
                    *current_op = Op::Change(move_dict(*m, map));
                    new_ops.push(new_op);
                }
                (Op::Move(m), Op::AddMul(or, oa, b)) => {
                    let new_op = Op::Move(*m);
                    *current_op = Op::AddMul(or + m, oa + m, *b);
                    new_ops.push(new_op);
                }
                // Compress Set/Change: TODO
                // Compress Change
                (Op::Change(ma), Op::Change(mb)) =>
                // Compress if they are the same
                {
                    let mut ma2 = ma.clone();

                    for (offset, value) in mb {
                        ma2.insert(*offset, value + *ma2.get(offset).unwrap_or(&0));
                    }
                    *current_op = Op::Change(ma2);
                }
                (Op::Set(ma), Op::Set(mb)) =>
                // Compress if they are the same
                {
                    let mut ma2 = ma.clone();

                    for (offset, value) in mb {
                        ma2.insert(*offset, *value);
                    }
                    *current_op = Op::Set(ma2);
                }
                // Compress Move
                (Op::Move(a), Op::Move(b)) => {
                    *current_op = Op::Move(*a + *b);
                }
                (_, b) => new_ops.push(b.clone()),
            },
        }
    }

    new_ops
}

pub fn optimise(mut ops: Vec<Op>) -> Vec<Op> {
    loop {
        let ops2 = optimise_single(ops.clone());
        let ops3 = optimise_peephole(ops2);

        if ops == ops3 {
            return ops;
        }
        ops = ops3;
    }
}
