use crate::syntax_ast;
use crate::syntax_ast::{Name, Statement};
use crate::token;
use crate::token::{Token, TokenType, TokenValue};

mod assignment;
mod conditional;
mod declaration;
mod expression;
mod file;
mod function;
mod r#loop;
mod r#match;
mod r#return;
mod statement;
mod type_annotation;
mod type_definition;
mod utils;

#[derive(Debug)]
pub(crate) enum ErrorType {
    Name,
    Module,
    Import,
    LineEnd,
    TypeDefinition,
    Declaration,
    TypeAnnotation,
    Expression,
    Statement,
    Conditional,
    Function,
    Match,
    Loop,
}

#[derive(Debug)]
pub(crate) struct Error {
    typ: ErrorType,
    msg: String,
    token: Option<Token>,
}

pub struct SyntacticParser {
    tokens: Vec<Token>,
    index: usize,
}

impl SyntacticParser {
    pub(crate) fn parse(
        tokens: Vec<Token>,
        filename: &str,
        module_name: &str,
    ) -> Result<syntax_ast::File, Error> {
        let mut parser = SyntacticParser {
            tokens,
            index: 0usize,
        };
        parser.parse_file(filename, module_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::syntax_ast::File;
    use insta;

    fn parse_test_file(code: &str, filename: &str, module_name: &str) -> File {
        let tokens = Lexer::lex(code).unwrap();
        SyntacticParser::parse(tokens, filename, module_name).unwrap()
    }

    #[test]
    fn test_basic() {
        let code = r#"module test_add;

import std;

prv fn add(a: i32, b: i32) -> i32 {
    let ret: i32 = a + b;
    return ret;
}

pub fn test() -> bool {
    let expected: i32 = 25;
    let result: i32 = add(30, -5);
    if (result == expected) {
        std::print("Passed!\n");
        return true;
    } else {
        std::print("Failed!\n");
        return false;
    }
}"#;
        let ast = parse_test_file(code, "test", "test_add");

        let mut settings = insta::Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            insta::assert_yaml_snapshot!(ast);
        });
    }
}
