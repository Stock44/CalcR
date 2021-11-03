#![feature(box_patterns)]
#![allow(unknown_lints)]

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);
mod ast;

#[cfg(test)]
#[allow(illegal_floating_point_literal_pattern)]
mod tests {
    use crate::ast::{Expression, Statement, Unit, Number, OpType};
    use super::*;

    fn match_stm_exp(stm: Statement) -> Expression {
        match stm {
            Statement::Expression(exp) => *exp,
            _ => panic!()
        }
    }

    #[test]
    fn integer_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("32").unwrap();
        let exp = match_stm_exp(result);
        assert!(matches!(exp, Expression::Constant{value: Number::Integer(32), units: None}));
    }

    #[test]
    fn decimal_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("32.5").unwrap();
        let exp = match_stm_exp(result);
        assert!(matches!(exp, Expression::Constant{value: Number::Decimal(32.5), units: None}));
    }

    #[test]
    fn number_w_units_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("32.5 kg m_1 s_-2").unwrap();

        let exp = match_stm_exp(result);
        match exp {
            Expression::Constant { value, units } => {
                assert_eq!(value, Number::Decimal(32.5));

                let units = units.unwrap();

                assert_eq!(units[0], Unit(String::from("kg"), 1));
                assert_eq!(units[1], Unit(String::from("m"), 1));
                assert_eq!(units[2], Unit(String::from("s"), -2));
            }
            _ => panic!()
        }
    }

    #[test]
    fn function_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("atan2(2, 3)").unwrap();
        let exp = match_stm_exp(result);
        match exp {
            Expression::Function { name, arguments } => {
                assert_eq!(name, String::from("atan2"));

                assert!(matches!(arguments[0], box Expression::Constant {
                    value: Number::Integer(2),
                    units: None
                }));
                assert!(matches!(arguments[1], box Expression::Constant {
                    value: Number::Integer(3),
                    units: None
                }));
            }
            _ => panic!()
        }
    }

    #[test]
    fn assignment_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("hello = 2.2").unwrap();
        let expected_name = String::from("hello");

        match result {
            Statement::Assignment { variable_name, value } => {
                assert_eq!(variable_name, expected_name);
                assert!(matches!(*value, Expression::Constant{value: Number::Decimal(2.2), units: None}));
            }
            _ => panic!()
        }
    }

    fn test_operation(exp: Expression, expected_op: OpType, expected_lhs: Expression, expected_rhs: Expression) {
        match exp {
            Expression::Operation {
                op_type, lhs, rhs
            } => {
                assert_eq!(op_type, expected_op);
                assert_eq!(expected_lhs, *lhs);
                assert_eq!(expected_rhs, *rhs);
            }
            _ => panic!()
        }
    }

    #[test]
    fn add_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("2 km_40 + 20.12 dm").unwrap();
        let exp = match_stm_exp(result);
        let expected_lhs = Expression::Constant { value: Number::Integer(2), units: Some(vec![Unit(String::from("km"), 40)]) };
        let expected_rhs = Expression::Constant { value: Number::Decimal(20.12), units: Some(vec![Unit(String::from("dm"), 1)]) };
        test_operation(exp, OpType::Add, expected_lhs, expected_rhs)
    }

    #[test]
    fn factor_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("4.52 cm * 2 cm_2").unwrap();
        let exp = match_stm_exp(result);
        let expected_lhs = Expression::Constant { value: Number::Decimal(4.52), units: Some(vec![Unit(String::from("cm"), 1)]) };
        let expected_rhs = Expression::Constant { value: Number::Integer(2), units: Some(vec![Unit(String::from("cm"), 2)]) };
        test_operation(exp, OpType::Multiply, expected_lhs, expected_rhs)
    }

    #[test]
    fn power_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("4.52 cm ^ 2").unwrap();
        let exp = match_stm_exp(result);
        let expected_lhs = Expression::Constant { value: Number::Decimal(4.52), units: Some(vec![Unit(String::from("cm"), 1)]) };
        let expected_rhs = Expression::Constant { value: Number::Integer(2), units: None };
        test_operation(exp, OpType::Power, expected_lhs, expected_rhs)
    }

    #[test]
    fn parentheses_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("(2 + 4) * 8").unwrap();
        let exp = match_stm_exp(result);

        match exp {
            Expression::Operation { op_type, lhs, rhs } => {
                let lhs = *lhs;
                let rhs = *rhs;

                assert!(matches!(lhs, Expression::Operation {
                    lhs: box Expression::Constant{value: Number::Integer(2),units: None},
                    rhs: box Expression::Constant {value: Number::Integer(4), units: None},
                    op_type: OpType::Add}));
                assert_eq!(rhs, Expression::Constant { value: Number::Integer(8), units: None });
                assert_eq!(op_type, OpType::Multiply);
            }
            _ => panic!()
        }
    }

    #[test]
    fn precedence_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("2 + 4 * 8 - 2 ^ 21").unwrap();
        let exp = match_stm_exp(result);

        assert!(matches!(exp, Expression::Operation {
            op_type: OpType::Subtract,
            lhs: box Expression::Operation {
                op_type: OpType::Add,
                lhs: box Expression::Constant {value: Number::Integer(2), units: None},
                rhs: box Expression::Operation {
                    op_type: OpType::Multiply,
                    lhs: box Expression::Constant {value: Number::Integer(4), units: None},
                    rhs: box Expression::Constant {value: Number::Integer(8), units: None},
                }
            },
            rhs: box Expression::Operation {
                op_type: OpType::Power,
                lhs: box Expression::Constant {value: Number::Integer(2), units: None},
                rhs: box Expression::Constant {value: Number::Integer(21), units: None},
            },
        }))
    }
}
