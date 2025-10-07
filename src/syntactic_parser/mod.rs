use crate::syntax_ast;
use crate::syntax_ast::Statement;
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

#[derive(Debug, Clone)]
pub(crate) enum ErrorType {
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
    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.index).cloned()
    }

    fn back(&self) -> Token {
        self.tokens[self.index - 1].clone()
    }

    fn error(&self, error_type: &ErrorType, message: &str) -> Error {
        Error {
            typ: error_type.clone(),
            msg: message.to_string(),
            token: self.peek(),
        }
    }

    fn expect_token(&self, error_type: &ErrorType, message: &str) -> Result<Token, Error> {
        match self.peek() {
            Some(token) => Ok(token),
            None => Err(self.error(error_type, message)),
        }
    }

    fn expect_identifier(&self, error_type: &ErrorType, message: &str) -> Result<String, Error> {
        let token = self.expect_token(error_type, message)?;
        match token.value {
            TokenValue::Identifier(id) => Ok(id),
            _ => Err(self.error(error_type, message)),
        }
    }

    fn expect_keyword(
        &self,
        kw: TokenType,
        error_type: &ErrorType,
        message: &str,
    ) -> Result<(), Error> {
        if !self.is_keyword(kw) {
            Err(self.error(error_type, message))
        } else {
            Ok(())
        }
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn is_keyword(&self, keyword: TokenType) -> bool {
        let Some(token) = self.peek() else {
            return false;
        };
        match token.value {
            TokenValue::Keyword(kwd) => kwd == keyword,
            _ => false,
        }
    }

    fn end_line(&mut self) -> Result<(), Error> {
        if !self.is_keyword(TokenType::Semicolon) {
            Err(self.error(&ErrorType::LineEnd, "`;` expected at end of line"))
        } else {
            self.advance();
            Ok(())
        }
    }

    fn is_identifier(&self) -> Option<String> {
        let token = self.peek()?;
        match token.value {
            TokenValue::Identifier(id) => Some(id),
            _ => None,
        }
    }

    fn is_mutable(&self) -> Result<bool, Error> {
        if self.is_keyword(TokenType::Let) {
            Ok(false)
        } else if self.is_keyword(TokenType::Var) {
            Ok(true)
        } else {
            Err(self.error(
                &ErrorType::TypeAnnotation,
                "Type annotations must specify mutability",
            ))
        }
    }

    fn is_uint(&self) -> Option<u64> {
        let token = self.peek()?;
        match token.value {
            TokenValue::Literal(token::Literal::UInt(uint)) => Some(uint),
            _ => None,
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, Error> {
        if !self.is_keyword(TokenType::OpenBracket) {
            return Err(self.error(&ErrorType::Conditional, "Expected contional body"));
        }
        self.advance();
        let mut statements = Vec::new();
        while !self.is_keyword(TokenType::CloseBracket) {
            statements.push(self.parse_statement()?);
        }
        self.advance();
        Ok(statements)
    }
}
