use crate::lexer::Lexer;
use crate::syntax_ast;
use crate::syntax_ast::{Name, Statement};
use crate::token;
use crate::token::{SymbolId, SymbolTable, Token, TokenType, TokenValue};

mod assignment;
mod conditional;
mod declaration;
mod expression;
mod file;
mod function;
mod r#loop;
mod r#match;
mod module;
mod r#return;
mod statement;
mod type_annotation;
mod type_definition;
mod utils;

#[derive(Debug)]
pub(crate) enum ErrorType {
    Lexer(Box<crate::lexer::Error>),
    Io(Box<std::io::Error>),
    ModuleFile(Box<serde_json::Error>),
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
    tokens: Vec<Token>,
    index: usize,
}

impl SyntacticParser {
    pub(crate) fn parse_code(
        code: &str,
        filename: SymbolId,
        module_name: SymbolId,
        symbol_table: &mut SymbolTable,
    ) -> Result<syntax_ast::File, Error> {
        let tokens = match Lexer::lex(code, symbol_table) {
            Ok(tokens) => tokens,
            Err(err) => {
                return Err(Error {
                    typ: ErrorType::Lexer(Box::new(err)),
                    msg: "Lexer error",
                    token: None,
                });
            }
        };
        let mut parser = Self { tokens, index: 0 };
        parser.parse_file(filename, module_name, symbol_table)
    }
}

#[cfg(test)]
mod tests {
    use crate::syntax_ast::File;

    use super::*;

    fn test_code(code: &str, filename: &str, module_name: &str) -> File {
        let mut symbol_table = SymbolTable::new();
        let filename = symbol_table.insert(filename.to_string());
        let module_name = symbol_table.insert(module_name.to_string());
        let ast =
            SyntacticParser::parse_code(code, filename, module_name, &mut symbol_table).unwrap();
        token::set_symbol_context(symbol_table);
        ast
    }

    #[test]
    fn basic() {
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
        let ast = test_code(code, "test", "test_add");
        let mut settings = insta::Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            insta::assert_yaml_snapshot!(ast);
        });
    }

    #[test]
    fn loops() {
        let code = r#"module test_loop;

import std;

pub fn count_bits(n: u32) -> u8 {
    var num: u32 = n;
    var count: u8 = 0;
    while (num > 0) {
        count += u8(num & 0b1);
        num >>= 1;
    }
    return count;
}

prv fn sum(list: []let i32) -> i32 {
    var ret: i32 = 0;
    for (var i: i32 = 0; i < list.len; i += 1) {
        ret += list[i];
    }
    return ret;
}

pub fn dead_loop() {
    while {
        std::print("Hello");
    }
}"#;
        let ast = test_code(code, "test", "test_loop");
        let mut settings = insta::Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            insta::assert_yaml_snapshot!(ast);
        });
    }

    #[test]
    fn types() {
        let code = r#"module test_types;

prv struct Point {
    x: i32,
    y: i32
}

pub union Person {
    student: Student,
    teacher: Teacher,
}

pub enum Color {
    Red,
    Blue = 5,
    Black = 8,
    Yellow,
}"#;
        let ast = test_code(code, "test", "test_types");
        let mut settings = insta::Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            insta::assert_yaml_snapshot!(ast);
        });
    }

    #[test]
    fn test_match() {
        let code = r#"module test_match;

import std;

pub fn is_true(cond: bool) -> bool {
    match (cond) {
        true => { return true; }
        false => { return false; }
        _ => { std::print("Never happends"); }
    }
}"#;
        let ast = test_code(code, "test", "test_match");
        let mut settings = insta::Settings::clone_current();
        settings.set_sort_maps(true);
        settings.bind(|| {
            insta::assert_yaml_snapshot!(ast);
        });
    }
}
