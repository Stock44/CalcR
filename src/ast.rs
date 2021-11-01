pub enum Statement {
    Expression(Box<Expression>),
    Assignment {
        variable_name: String,
        value: Box<Expression>,
    },
}




pub enum Expression {
    Number {
        value: f64,
        unit: Option<String>,
    },
    Variable {
        name: String,
    },
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

pub enum OpType {
    Multiplication,
    Division,
    Subtraction,
    Addition,
    Floor,
    Modulo,
    Power,
}
