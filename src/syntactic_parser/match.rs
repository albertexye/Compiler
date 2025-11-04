use super::*;
use syntax_ast::{ConditionalBranch, Match};

impl SyntacticParser {
    pub(super) fn parse_match(
        &mut self,
        symbol_table: &mut SymbolTable,
    ) -> Result<Statement, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Match));
        self.advance();
        self.expect_keyword(
            TokenType::OpenParen,
            ErrorType::Match,
            "Expected match value",
        )?;
        self.advance();
        let value = self.parse_expression()?;
        self.expect_keyword(TokenType::CloseParen, ErrorType::Match, "Expected `)`")?;
        self.advance();
        self.expect_keyword(
            TokenType::OpenBracket,
            ErrorType::Match,
            "Expected match body",
        )?;
        self.advance();
        let mut cases = Vec::new();
        let mut default = None;
        while !self.is_keyword(TokenType::CloseBracket) {
            if let Some(id) = self.is_identifier()
                && Some(id) == symbol_table.search("_")
            {
                if default.is_some() {
                    return Err(self.error(ErrorType::Match, "Multiple default branches"));
                }
                self.advance();
                default = Some(self.parse_case_body(symbol_table)?);
            } else {
                cases.push(self.parse_case(symbol_table)?);
            }
        }
        self.advance();
        Ok(Statement::Match(Match {
            value,
            cases,
            default,
        }))
    }

    fn parse_case(&mut self, symbol_table: &mut SymbolTable) -> Result<ConditionalBranch, Error> {
        let condition = self.parse_expression()?;
        Ok(ConditionalBranch {
            condition,
            body: self.parse_case_body(symbol_table)?,
        })
    }

    fn parse_case_body(&mut self, symbol_table: &mut SymbolTable) -> Result<Vec<Statement>, Error> {
        self.expect_keyword(TokenType::MatchCase, ErrorType::Match, "Expected case")?;
        self.advance();
        self.parse_block(symbol_table)
    }
}
