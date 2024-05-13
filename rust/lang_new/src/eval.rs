use super::ast;

pub fn eval_program(program: ast::Program) -> Vec<bool> {
    let mut res = Vec::new();
    for i in program.into_vec() {
        res.push(eval_node(&i))
    }
    res
}

fn eval_node(node: &ast::Node) -> bool {
    match node {
        ast::Node::Value(n) => *n,
        ast::Node::Expr { op, args } => {
            let lhs_ret = eval_node(&args[0]);
            let rhs_ret = eval_node(&args[1]);

            match op {
                ast::Operator::And => lhs_ret && rhs_ret,
                ast::Operator::Or => lhs_ret || rhs_ret,
            }
        }
    }
}
