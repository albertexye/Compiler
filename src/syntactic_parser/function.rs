use super::*;
use syntax_ast::{Function, FunctionArg, TypeAnnot};

impl SyntacticParser {
    pub(super) fn parse_function(&mut self) -> Result<Function, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Fn));
        self.advance();
        let Some(name) = self.is_identifier() else {
            return Err(self.error(ErrorType::Function, "Expected function name"));
        };
        let span = self.peek().unwrap().span;
        self.advance();
        let arguments = self.parse_arguments()?;
        let return_typ = self.parse_return_type()?;
        let body = self.parse_block()?;
        Ok(Function {
            name,
            arguments,
            return_type: return_typ,
            body,
            span,
        })
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
