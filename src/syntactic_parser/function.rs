use super::*;
use syntax_ast::{Function, FunctionArg, FunctionBody, TypeAnnot};

impl SyntacticParser {
    pub(super) fn parse_function(&mut self) -> Result<Function, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Fn) || self.is_keyword(TokenType::Asm));
        let is_fn = self.is_keyword(TokenType::Fn);
        self.advance();
        let Some(name) = self.is_identifier() else {
            return Err(self.error(ErrorType::Function, "Expected function name"));
        };
        let span = self.peek().unwrap().span;
        self.advance();
        let arguments = self.parse_arguments()?;
        let return_typ = self.parse_return_type()?;
        let body = if is_fn {
            FunctionBody::Normal(self.parse_block()?)
        } else {
            FunctionBody::Asm(self.parse_asm()?)
        };
        Ok(Function {
            name,
            arguments,
            return_type: return_typ,
            body,
            span,
        })
    }

    fn parse_asm(&mut self) -> Result<String, Error> {
        self.expect_keyword(TokenType::OpenParen, ErrorType::Function, "Expected `{`")?;
        self.advance();
        let mut lines = Vec::new();
        while let Some(token) = self.peek() {
            match token.value {
                TokenValue::Literal(token::Literal::String(string)) => lines.push(string),
                _ => break,
            }
            self.advance();
        }
        self.expect_keyword(TokenType::CloseParen, ErrorType::Function, "Expected `}`")?;
        Ok(lines.join("\n"))
    }

    fn parse_return_type(&mut self) -> Result<Option<TypeAnnot>, Error> {
        if self.is_keyword(TokenType::OpenBracket) {
            return Ok(None);
        }
        if !self.is_keyword(TokenType::ReturnType) {
            return Err(self.error(ErrorType::Function, "Expected function name"));
        }
        self.advance();
        Ok(Some(self.parse_type_annotation()?))
    }

    fn parse_arguments(&mut self) -> Result<Vec<FunctionArg>, Error> {
        if !self.is_keyword(TokenType::OpenParen) {
            return Err(self.error(ErrorType::Function, "Expected argument list"));
        }
        self.advance();
        let mut arguments = Vec::new();
        while !self.is_keyword(TokenType::CloseParen) {
            arguments.push(self.parse_argument()?);
            if self.is_keyword(TokenType::CloseParen) {
                break;
            }
            if !self.is_keyword(TokenType::Comma) {
                return Err(self.error(ErrorType::Function, "Expected `)`"));
            }
            self.advance();
        }
        self.advance();
        Ok(arguments)
    }

    fn parse_argument(&mut self) -> Result<FunctionArg, Error> {
        let Some(name) = self.is_identifier() else {
            return Err(self.error(ErrorType::Function, "Expected argument name"));
        };
        let start = self.peek().unwrap().span;
        self.advance();
        if !self.is_keyword(TokenType::Colon) {
            return Err(self.error(ErrorType::Function, "Argument type must be specified"));
        }
        self.advance();
        let typ = self.parse_type_annotation()?;
        let end = self.back().span;
        Ok(FunctionArg {
            name,
            typ,
            span: end - start,
        })
    }
}
