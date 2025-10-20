use super::*;

impl SyntacticParser {
    pub(super) fn parse_return(&mut self) -> Result<Statement, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Return));
        self.advance();
        let exp = self.parse_expression()?;
        self.end_line()?;
        Ok(Statement::Return(exp))
    }
}
