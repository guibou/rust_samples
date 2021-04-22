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
                (Op::Change(a), Op::Change(b)) => *current_op = Op::Change(*a + *b),
                (Op::Move(a), Op::Move(b)) => *current_op = Op::Move(*a + *b),
                (_, b) => new_ops.push(b.clone()),
            },
        }
    }

    new_ops
}

pub fn optimise(ops: Vec<Op>) -> Vec<Op> {
    optimise_peephole(optimise_single(ops))
}
