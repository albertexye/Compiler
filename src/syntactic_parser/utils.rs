use super::*;

impl SyntacticParser {
    pub(super) fn peek(&self) -> Option<Token> {
        self.tokens.get(self.index).cloned()
    }

    pub(super) fn back(&self) -> Token {
        self.tokens[self.index - 1].clone()
    }

    pub(super) fn error(&self, error_type: ErrorType, message: &str) -> Error {
        Error {
            typ: error_type,
            msg: message.to_string(),
            token: self.peek(),
        }
    }

    pub(super) fn expect_token(
        &self,
        error_type: ErrorType,
        message: &str,
    ) -> Result<Token, Error> {
        match self.peek() {
            Some(token) => Ok(token),
            None => Err(self.error(error_type, message)),
        }
    }

    pub(super) fn expect_identifier(
        &self,
        error_type: ErrorType,
        message: &str,
    ) -> Result<String, Error> {
        let token = match self.peek() {
            Some(token) => token,
            None => return Err(self.error(error_type, message)),
        };
        match token.value {
            TokenValue::Identifier(id) => Ok(id),
            _ => Err(self.error(error_type, message)),
        }
    }

    pub(super) fn expect_keyword(
        &self,
        kw: TokenType,
        error_type: ErrorType,
        message: &str,
    ) -> Result<(), Error> {
        if !self.is_keyword(kw) {
            Err(self.error(error_type, message))
        } else {
            Ok(())
        }
    }

    pub(super) fn advance(&mut self) {
        self.index += 1;
    }

    pub(super) fn is_keyword(&self, keyword: TokenType) -> bool {
        let Some(token) = self.peek() else {
            return false;
        };
        match token.value {
            TokenValue::Keyword(kwd) => kwd == keyword,
            _ => false,
        }
    }

    pub(super) fn end_line(&mut self) -> Result<(), Error> {
        if !self.is_keyword(TokenType::Semicolon) {
            Err(self.error(ErrorType::LineEnd, "`;` expected at end of line"))
        } else {
            self.advance();
            Ok(())
        }
    }

    pub(super) fn is_identifier(&self) -> Option<String> {
        let token = self.peek()?;
        match token.value {
            TokenValue::Identifier(id) => Some(id),
            _ => None,
        }
    }

    pub(super) fn is_mutable(&self) -> Result<bool, Error> {
        if self.is_keyword(TokenType::Let) {
            Ok(false)
        } else if self.is_keyword(TokenType::Var) {
            Ok(true)
        } else {
            Err(self.error(
                ErrorType::TypeAnnotation,
                "Type annotations must specify mutability",
            ))
        }
    }

    pub(super) fn is_uint(&self) -> Option<u64> {
        let token = self.peek()?;
        match token.value {
            TokenValue::Literal(token::Literal::UInt(uint)) => Some(uint),
            _ => None,
        }
    }

    pub(super) fn parse_block(&mut self) -> Result<Vec<Statement>, Error> {
        if !self.is_keyword(TokenType::OpenBracket) {
            return Err(self.error(ErrorType::Conditional, "Expected contional body"));
        }
        self.advance();
        let mut statements = Vec::new();
        while !self.is_keyword(TokenType::CloseBracket) {
            statements.push(self.parse_statement()?);
        }
        self.advance();
        Ok(statements)
    }

    pub(super) fn parse_name(&mut self) -> Result<Name, Error> {
        std::debug_assert!(self.is_identifier().is_some());
        let mut name = Vec::new();
        name.push(self.is_identifier().unwrap());
        self.advance();
        while self.is_keyword(TokenType::DoubleColon) {
            self.advance();
            if let Some(id) = self.is_identifier() {
                name.push(id);
                self.advance();
            } else {
                return Err(self.error(ErrorType::Expression, "Expected identifier"));
            }
        }
        Ok(name)
    }
}
