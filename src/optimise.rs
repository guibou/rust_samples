use crate::ast::Op;
use std::collections::HashMap;

// Optimise single instructions
pub fn optimise_single(ops: Vec<Op>) -> Vec<Op> {
    let mut new = Vec::new();

    for o in ops {
        match o {
            Op::Move(0) => (),
            _ => new.push(match o {
            Op::Loop(l) => match l.as_slice() {
                [Op::Change(m)] => {
                    if m.len() == 1 && m.get(&0) == Some(&-1) {
                        let mut set_map = HashMap::new();
                        set_map.insert(0, 0);
                        Op::Set(set_map)
                    } else {
                        Op::Loop(optimise(l))
                    }
                }
                _ => Op::Loop(optimise(l)),
            },
            _ => o,
        }),
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
                // Cancels Set: TODO
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
