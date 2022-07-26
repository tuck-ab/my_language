#[derive(Debug, PartialEq)]
pub struct Program{
    pub program: Block
}

impl Program {
    pub fn new() -> Self {
        Self{program: Block{statements: Vec::new()}}
    }
}

#[derive(Debug, PartialEq)]
pub struct Block{
    pub statements: Vec<Statement>
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    AssignStatement{
        var: String, 
        exp: Expression
    },
    IfStatement{
        condition: Expression,
        body: Block,
        else_if: Vec<(Expression,Block)>,
        else_body: Block
    },
    RepeatStatement{
        times: Expression,
        body: Block
    },
    OutputStatement{
        to_output: Expression
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Val(i32),
    Var(String),
    BinOp(Op, Box<Expression>, Box<Expression>)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Multiply,
    Divide,
    Remainder,
    And,
    Or,
    Equal,
    NotEqual,
    LessThanOrEqual,
    LessThan,
    GreaterThanOrEqual,
    GreaterThan
}
