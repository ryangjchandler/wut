pub type Program = Vec<Statement>;
pub type Block = Vec<Statement>;

#[derive(Debug)]
pub enum Statement {
    Function {
        name: String,
        args: Vec<String>,
        body: Block,
        exported: bool,
    },
    If {
        condition: Expression,
        then: Block,
        otherwise: Option<Block>,
    },
    Return {
        value: Option<Expression>,
    }
}

#[derive(Debug)]
pub enum Expression {
    Add {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Subtract {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    LessThan {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Call {
        target: String,
        args: Vec<Expression>,
    },
    Identifier {
        name: String,
    },
    Integer {
        value: String,
    }
}