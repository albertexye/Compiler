use crate::lexer::Lexer;
use crate::syntax_ast;
use crate::syntax_ast::{Name, Statement};
use crate::token;
use crate::token::{SymbolId, Token, TokenType, TokenValue};

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
    Lexer(Box<crate::lexer::Error>),
    Io(Box<std::io::Error>),
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
    msg: &'static str,
    token: Option<Token>,
}

pub struct SyntacticParser {
    lexer: Lexer,
    tokens: Vec<Token>,
    index: usize,
}

impl SyntacticParser {
    pub(crate) fn new() -> SyntacticParser {
        SyntacticParser {
            lexer: Lexer::new(),
            tokens: Vec::new(),
            index: 0,
        }
    }

    pub(crate) fn parse(
        &mut self,
        code: &str,
        filename: &str,
        module_name: &str,
    ) -> Result<syntax_ast::File, Error> {
        self.tokens = match self.lexer.lex(code) {
            Ok(tokens) => tokens,
            Err(err) => {
                return Err(Error {
                    typ: ErrorType::Lexer(Box::new(err)),
                    msg: "Lexer error",
                    token: None,
                });
            }
        };
        let ret = self.parse_file(filename, module_name);
        self.tokens = Vec::new();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax_ast::File;

    fn parse_test_file(code: &str, filename: &str, module_name: &str) -> File {
        let mut parser = SyntacticParser::new();
        let ret = parser.parse(code, filename, module_name).unwrap();
        token::set_symbol_context(parser.lexer.symbol_table);
        ret
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
