#[derive(PartialEq, Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    Assignment {
        variable_name: String,
        value: Box<Expression>,
    },
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Constant {
        value: Number,
        units: Option<Vec<Unit>>,
    },
    Variable(String),
    Function {
        name: String,
        arguments: Vec<Box<Expression>>,
    },
    Operation {
        op_type: OpType,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Conversion {
        target_unit: Unit,
        value: Box<Expression>,
    },
}

#[derive(PartialEq, Debug)]
pub struct Unit(pub String, pub i64);

#[derive(PartialEq, Debug)]
pub enum OpType {
    Multiply,
    Divide,
    Subtract,
    Add,
    Floor,
    Modulo,
    Power,
}

#[derive(PartialEq, Debug)]
pub enum Number {
    Decimal(f64),
    Integer(i64),
}
