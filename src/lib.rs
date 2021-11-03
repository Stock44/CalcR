#![feature(box_patterns)]

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);
mod ast;

#[cfg(test)]
#[allow(illegal_floating_point_literal_pattern)]
mod tests {
    use crate::ast::{Expression, Statement, Unit, NumType};
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
        assert!(matches!(exp, Expression::Constant{value: NumType::Integer(32), units: None}));
    }

    #[test]
    fn decimal_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("32.5").unwrap();
        let exp = match_stm_exp(result);
        assert!(matches!(exp, Expression::Constant{value: NumType::Decimal(32.5), units: None}));
    }

    #[test]
    fn number_w_units_parsing() {
        let parser = grammar::StatementParser::new();
        let result: Statement = parser.parse("32.5 kg m_1 s_-2").unwrap();

        let exp = match_stm_exp(result);
        match exp {
            Expression::Constant { value, units } => {
                assert_eq!(value, NumType::Decimal(32.5));

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
                    value: NumType::Integer(2),
                    units: None
                }));
                assert!(matches!(arguments[1], box Expression::Constant {
                    value: NumType::Integer(3),
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
                assert!(matches!(*value, Expression::Constant{value: NumType::Decimal(2.2), units: None}));
            }
            _ => panic!()
        }
    }
}
