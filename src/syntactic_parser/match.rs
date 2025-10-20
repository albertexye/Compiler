use super::*;
use syntax_ast::{ConditionalBranch, Match};

impl SyntacticParser {
    pub(super) fn parse_match(&mut self) -> Result<Statement, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Match));
        self.advance();
        let value = self.parse_expression()?;
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
                && &id == "_"
            {
                if default.is_some() {
                    return Err(self.error(ErrorType::Match, "Multiple default branches"));
                }
                self.advance();
                default = Some(self.parse_case_body()?);
            } else {
                cases.push(self.parse_case()?);
            }
        }
        self.advance();
        Ok(Statement::Match(Match {
            value,
            cases,
            default,
        }))
    }

    fn parse_case(&mut self) -> Result<ConditionalBranch, Error> {
        let condition = self.parse_expression()?;
        Ok(ConditionalBranch {
            condition,
            body: self.parse_case_body()?,
        })
    }

    fn parse_case_body(&mut self) -> Result<Vec<Statement>, Error> {
        self.expect_keyword(TokenType::MatchCase, ErrorType::Match, "Expected case")?;
        self.advance();
        self.parse_block()
    }
}
