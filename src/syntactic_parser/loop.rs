use super::*;
use syntax_ast::Loop;

impl SyntacticParser {
    pub(crate) fn parse_loop(&mut self) -> Result<Statement, Error> {
        Ok(Statement::Loop(if self.is_keyword(TokenType::For) {
            self.parse_for()?
        } else if self.is_keyword(TokenType::While) {
            self.parse_while()?
        } else {
            panic!("Invalid loop keyword");
        }))
    }

    fn parse_for(&mut self) -> Result<Loop, Error> {
        std::debug_assert!(self.is_keyword(TokenType::For));
        self.advance();
        let initialization = if self.is_keyword(TokenType::Semicolon) {
            self.advance();
            None
        } else {
            Some(self.parse_declaration()?)
        };
        let condition = if self.is_keyword(TokenType::Semicolon) {
            self.advance();
            None
        } else {
            Some(self.parse_expression()?)
        };
        let mut update = Vec::new();
        if !self.is_keyword(TokenType::OpenBracket) {
            update.push(self.parse_assignment_or_expression()?);
            while self.is_keyword(TokenType::Comma) {
                self.advance();
                update.push(self.parse_assignment_or_expression()?);
            }
        }
        let body = self.parse_block()?;
        Ok(Loop {
            init: initialization,
            condition,
            update,
            body,
        })
    }

    fn parse_while(&mut self) -> Result<Loop, Error> {
        std::debug_assert!(self.is_keyword(TokenType::While));
        self.advance();
        let condition = if !self.is_keyword(TokenType::OpenBracket) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        let body = self.parse_block()?;
        Ok(Loop {
            condition,
            init: None,
            update: Vec::new(),
            body,
        })
    }
}
