use chumsky::prelude::*;



pub fn parse(input: &str) -> Result<JqChain, Error> {
    let mut parser = Parser::new(input);
    let result = parser.parse();
    result
}
