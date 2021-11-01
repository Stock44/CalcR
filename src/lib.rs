mod ast;
#[macro_use] extern crate  lalrpop_util;


#[cfg(test)]
mod tests {


    lalrpop_mod!(pub grammar); // synthesized by LALRPOP
    #[test]
    fn it_works() {

        let result = grammar::StatementParser::new().parse("hello = 2.12");
        assert!(result.is_ok());
    }
}
