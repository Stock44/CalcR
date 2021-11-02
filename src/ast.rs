pub enum Statement {
    Expression(Box<Expression>),
    Assignment {
        variable_name: String,
        value: Box<Expression>,
    },
}

pub enum Expression {
    Constant {
        value: NumType,
        units: Option<Vec<Unit>>,
    },
    Variable(String),
    Function {
        name: String,
        arguments: Vec<Box<Expression>>,
    },
    Operation {
        op_type: OpType,
        x: Box<Expression>,
        y: Box<Expression>,
    },
    Conversion {
        target_unit: String,
        value: Box<Expression>,
    },
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Unit(pub String, pub i64);

pub enum OpType {
    Multiplication,
    Division,
    Subtraction,
    Addition,
    Floor,
    Modulo,
    Power,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum NumType {
    Decimal(f64),
    Integer(i64),
}
