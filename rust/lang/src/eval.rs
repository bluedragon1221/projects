use super::ast::*;

pub fn eval_program(program: Program) -> Vec<bool> {
    let mut res = Vec::new();
    for i in program.into_vec() {
        res.push(eval_node(&i))
    }
    res
}

fn eval_node(node: &Node) -> bool {
    match node {
        Node::Value(n) => *n,
        Node::Expr(Expr { op, lhs, rhs }) => {
            let lhs_ret = eval_node(lhs);
            let rhs_ret = eval_node(rhs);

            match op {
                Operator::And => lhs_ret && rhs_ret,
                Operator::Or => lhs_ret || rhs_ret,
            }
        }
    }
}
