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
