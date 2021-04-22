use crate::ast::Op;

// Optimise single instructions
pub fn optimise_single(ops: Vec<Op>) -> Vec<Op> {
    let mut new = Vec::new();

    for o in ops {
        new.push(match o {
            Op::Loop(l) => Op::Loop(optimise(l)),
            _ => o,
        })
    }

    new
}

// optimise_peephole
pub fn optimise_peephole(ops: Vec<Op>) -> Vec<Op> {
    let mut new_ops: Vec<Op> = Vec::new();

    for op in ops {
        match new_ops.last_mut() {
            None => new_ops.push(op.clone()),
            Some(current_op) => match (&current_op, &op) {
                // Move the move to the right
                (Op::Move(m), Op::Change(v, doffset)) => {
                    let new_op = Op::Move(*m);
                    *current_op = Op::Change(*v, doffset + m);
                    new_ops.push(new_op);
                }
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
                // Compress Change and sort them by order
                (Op::Change(a, da), Op::Change(b, db)) =>
                // Compress if they are the same
                {
                    if *da == *db {
                        *current_op = Op::Change(*a + *b, *da);
                    } else {
                        // Sort by offset
                        if da < db {
                            new_ops.push(op.clone());
                        } else {
                            let new_op = current_op.clone();
                            *current_op = op.clone();
                            new_ops.push(new_op);
                        }
                    }
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
