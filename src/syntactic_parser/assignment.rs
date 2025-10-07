use super::*;
use syntax_ast::{Assignment, AssignmentType};

impl SyntacticParser {
    pub(crate) fn parse_assignment_or_expression(&mut self) -> Result<Statement, Error> {
        let start = self.peek().unwrap().span;
        let left = self.parse_expression()?;
        let token = self.expect_token(&ErrorType::Statement, "Invalid statement")?;
        let TokenValue::Keyword(punc) = token.value else {
            return Err(self.error(&ErrorType::Statement, "Expected assignment operator"));
        };
        if punc == TokenType::Semicolon {
            self.advance();
            return Ok(Statement::Expression(left));
        }
        let typ = SyntacticParser::match_assignment_type(punc)
            .ok_or(self.error(&ErrorType::Statement, "Invalid expression"))?;
        self.advance();
        let right = self.parse_expression()?;
        let end = self.peek();
        self.end_line()?;
        let end = end.unwrap().span;
        Ok(Statement::Assignment(Assignment {
            left,
            right,
            typ,
            span: end - start,
        }))
    }

    fn match_assignment_type(punc: token::TokenType) -> Option<AssignmentType> {
        Some(match punc {
            TokenType::Assign => AssignmentType::Assign,
            TokenType::PlusEq => AssignmentType::Plus,
            TokenType::MinusEq => AssignmentType::Minus,
            TokenType::MulEq => AssignmentType::Mul,
            TokenType::DivEq => AssignmentType::Div,
            TokenType::ModEq => AssignmentType::Mod,
            TokenType::LeftShiftEq => AssignmentType::LeftShift,
            TokenType::RightShiftEq => AssignmentType::RightShift,
            TokenType::BitAndEq => AssignmentType::BitAnd,
            TokenType::BitOrEq => AssignmentType::BitOr,
            TokenType::BitXorEq => AssignmentType::BitXor,
            _ => return None,
        })
    }
}
