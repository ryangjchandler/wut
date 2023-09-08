use lalrpop_util::lalrpop_mod;

use crate::ast::Program;

lalrpop_mod!(grammar);

pub fn parse(text: &str) -> Program {
    let parser = grammar::ProgramParser::new();
    parser.parse(text).unwrap()
}