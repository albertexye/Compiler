use std::collections::HashMap;

use crate::syntax_ast::ExpressionValue;

use super::*;
use syntax_ast::{Binary, BinaryOp, Call, Expression, Unary, UnaryOp};

impl SyntacticParser {
    pub(crate) fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.pratt_parse(0)
    }

    fn parse_paren(&mut self) -> Result<Expression, Error> {
        let exp = self.pratt_parse(0)?;
        if !self.is_keyword(TokenType::CloseParen) {
            return Err(self.error(ErrorType::Expression, "Unclosed parenthesis"));
        }
        self.advance();
        Ok(exp)
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Result<Vec<Expression>, Error> {
        let mut list = Vec::new();
        loop {
            if self.is_keyword(end) {
                break;
            }
            list.push(self.parse_expression()?);
            if self.is_keyword(end) {
                break;
            }
            if !self.is_keyword(TokenType::Comma) {
                return Err(self.error(ErrorType::Expression, "Expected `,`"));
            }
            self.advance();
        }
        self.advance();
        Ok(list)
    }

    fn parse_array_literal(&mut self) -> Result<Expression, Error> {
        let start = self.back().span;
        let ev = ExpressionValue::Literal(syntax_ast::Literal::Array(
            self.parse_expression_list(TokenType::CloseBracket)?,
        ));
        let end = self.back().span;
        Ok(Expression {
            value: ev,
            span: end - start,
        })
    }

    fn parse_struct_literal(&mut self) -> Result<HashMap<String, Expression>, Error> {
        let mut ret = HashMap::new();
        loop {
            if self.is_keyword(TokenType::CloseBracket) {
                break;
            }
            let field = self.expect_identifier(ErrorType::Expression, "Expected field name")?;
            self.advance();
            if !self.is_keyword(TokenType::Colon) {
                return Err(self.error(ErrorType::Expression, "Expected `:`"));
            }
            self.advance();
            let exp = self.parse_expression()?;
            ret.insert(field, exp);
            if self.is_keyword(TokenType::CloseBracket) {
                break;
            }
            if !self.is_keyword(TokenType::Comma) {
                return Err(self.error(ErrorType::Expression, "Expected `,`"));
            }
        }
        self.advance();
        Ok(ret)
    }

    fn parse_infix_op(&mut self, punc: TokenType) -> Result<Expression, Error> {
        let start = self.back().span;
        let op = match punc {
            TokenType::Minus => UnaryOp::Negate,
            TokenType::Mul => UnaryOp::Dereference,
            TokenType::BitAnd => UnaryOp::AddressOf,
            TokenType::BitNot => UnaryOp::BitNot,
            TokenType::LogicalNot => UnaryOp::LogicalNot,
            _ => {
                return Err(self.error(ErrorType::Expression, "Invalid unary operator"));
            }
        };
        let operand = Box::new(self.pratt_parse(100)?);
        let end = self.back().span;
        Ok(Expression {
            value: ExpressionValue::Unary(Unary { op, operand }),
            span: end - start,
        })
    }

    fn parse_prefix(&mut self) -> Result<Expression, Error> {
        let token = self.expect_token(ErrorType::Expression, "No expression found")?;
        let start = token.span;
        Ok(match token.value {
            TokenValue::Identifier(_) => Expression {
                value: ExpressionValue::Identifier(self.parse_name()?),
                span: start - self.back().span,
            },
            TokenValue::Literal(literal) => {
                self.advance();
                Expression {
                    value: ExpressionValue::Literal(match literal {
                        token::Literal::UInt(uint) => syntax_ast::Literal::UInt(uint),
                        token::Literal::Int(int) => syntax_ast::Literal::Int(int),
                        token::Literal::Float(float) => syntax_ast::Literal::Float(float),
                        token::Literal::String(string) => syntax_ast::Literal::String(string),
                    }),
                    span: start - self.back().span,
                }
            }
            TokenValue::Keyword(punc) => {
                self.advance();
                match punc {
                    TokenType::OpenParen => self.parse_paren()?,
                    TokenType::OpenBracket => self.parse_array_literal()?,
                    _ => self.parse_infix_op(punc)?,
                }
            }
        })
    }

    fn is_postfix_op(punc: TokenType) -> bool {
        matches!(
            punc,
            TokenType::OpenParen | TokenType::OpenBrace | TokenType::OpenBracket
        )
    }

    fn parse_postfix(&mut self, punc: TokenType, left: Expression) -> Result<Expression, Error> {
        let start = self.back().span;
        let ev = match punc {
            TokenType::OpenParen => ExpressionValue::Call(Call {
                function: Box::new(left),
                args: self.parse_expression_list(TokenType::CloseParen)?,
            }),
            TokenType::OpenBrace => {
                let exp = ExpressionValue::Binary(Binary {
                    op: BinaryOp::Indexing,
                    left: Box::new(left),
                    right: Box::new(self.parse_expression()?),
                });
                if !self.is_keyword(TokenType::CloseParen) {
                    return Err(self.error(ErrorType::Expression, "Expected `]`"));
                }
                self.advance();
                exp
            }
            TokenType::OpenBracket => {
                ExpressionValue::Literal(syntax_ast::Literal::Struct(self.parse_struct_literal()?))
            }
            _ => panic!("Not a postfix operator"),
        };
        let end = self.back().span;
        Ok(Expression {
            value: ev,
            span: end - start,
        })
    }

    fn pratt_parse(&mut self, left_precedence: u8) -> Result<Expression, Error> {
        let mut exp = self.parse_prefix()?;
        loop {
            let Some(token) = self.peek() else {
                return Ok(exp);
            };
            let start = token.span;
            self.advance();
            let TokenValue::Keyword(punc) = token.value else {
                return Err(self.error(ErrorType::Expression, "Expected an operator"));
            };
            if SyntacticParser::is_postfix_op(punc) {
                exp = self.parse_postfix(punc, exp)?;
                continue;
            }
            let Some((precedence, op)) = SyntacticParser::match_infix_operator(punc) else {
                return Ok(exp);
            };
            if precedence < left_precedence {
                return Ok(exp);
            }
            self.advance();
            let right = Box::new(self.pratt_parse(precedence)?);
            let end = self.back().span;
            exp = Expression {
                value: ExpressionValue::Binary(Binary {
                    left: Box::new(exp),
                    right,
                    op,
                }),
                span: end - start,
            };
        }
    }

    fn match_infix_operator(infix: TokenType) -> Option<(u8, BinaryOp)> {
        Some(match infix {
            TokenType::Dot => (100, BinaryOp::FieldAccess),
            TokenType::Mul => (90, BinaryOp::Mul),
            TokenType::Div => (90, BinaryOp::Div),
            TokenType::Modulo => (90, BinaryOp::Mod),
            TokenType::Plus => (80, BinaryOp::Plus),
            TokenType::Minus => (80, BinaryOp::Minus),
            TokenType::LeftShift => (70, BinaryOp::LeftShift),
            TokenType::RightShift => (70, BinaryOp::RightShift),
            TokenType::BitAnd => (60, BinaryOp::BitAnd),
            TokenType::BitOr => (60, BinaryOp::BitOr),
            TokenType::BitXor => (60, BinaryOp::BitXor),
            TokenType::Eq => (50, BinaryOp::Eq),
            TokenType::NotEq => (50, BinaryOp::NotEq),
            TokenType::Gt => (50, BinaryOp::Gt),
            TokenType::Ge => (50, BinaryOp::Ge),
            TokenType::Lt => (50, BinaryOp::Lt),
            TokenType::Le => (50, BinaryOp::Le),
            TokenType::LogicalAnd => (40, BinaryOp::LogicalAnd),
            TokenType::LogicalOr => (40, BinaryOp::LogicalOr),
            _ => return None,
        })
    }
}
