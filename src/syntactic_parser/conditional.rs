use super::*;
use syntax_ast::{Conditional, ConditionalBranch};

impl SyntacticParser {
    pub(super) fn parse_conditional(&mut self) -> Result<Statement, Error> {
        std::debug_assert!(self.is_keyword(TokenType::If));
        self.advance();
        let if_condition = self.parse_paren_exp()?;
        let if_block = self.parse_block()?;
        let if_branch = ConditionalBranch {
            condition: if_condition,
            body: if_block,
        };
        let mut elif_branches = Vec::new();
        let mut else_branch = None;
        while self.is_keyword(TokenType::Else) {
            self.advance();
            if self.is_keyword(TokenType::If) {
                self.advance();
                let elif_condition = self.parse_paren_exp()?;
                let elif_block = self.parse_block()?;
                elif_branches.push(ConditionalBranch {
                    condition: elif_condition,
                    body: elif_block,
                });
            } else {
                else_branch = Some(self.parse_block()?);
                break;
            }
        }
        Ok(Statement::Conditional(Conditional {
            if_branch,
            elif_branches,
            else_branch,
        }))
    }
}
