use super::*;

impl SyntacticParser {
    pub(super) fn parse_statement(
        &mut self,
        pool: &mut InternPool,
    ) -> Result<Statement, Error> {
        let token = self.expect_token(ErrorType::Statement, "Expected statement")?;
        let TokenValue::Keyword(kw) = token.value else {
            return self.parse_assignment_or_expression(true);
        };
        match kw {
            TokenType::If => self.parse_conditional(pool),
            TokenType::Match => self.parse_match(pool),
            TokenType::For | TokenType::While => self.parse_loop(pool),
            TokenType::Let | TokenType::Var => {
                Ok(Statement::Declaration(self.parse_declaration()?))
            }
            TokenType::Return => self.parse_return(),
            TokenType::Continue => Ok(Statement::Continue(token.span)),
            TokenType::Break => Ok(Statement::Break(token.span)),
            _ => Err(self.error(ErrorType::Statement, "Invalid statement")),
        }
    }
}
