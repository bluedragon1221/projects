#[derive(Debug)]
pub enum Operator {
    And,
    Or,
}

#[derive(Debug)]
pub enum Node {
    Value(bool),
    Expr { op: Operator, args: Vec<Box<Node>> },
}

#[derive(Debug)]
pub struct Program(Vec<Node>);
impl Program {
    pub fn new(a: Vec<Node>) -> Self {
        Self(a)
    }
    pub fn into_vec(self) -> Vec<Node> {
        self.0
    }
}
