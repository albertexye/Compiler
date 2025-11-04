use crate::syntax_ast::{FunctionSig, TypeAnnotBase};

use super::*;
use syntax_ast::{TypeAnnot, TypeModifier, TypeModifierType};

impl SyntacticParser {
    pub(super) fn parse_type_annotation(&mut self) -> Result<TypeAnnot, Error> {
        let mut modifiers = Vec::new();
        let start = self.peek();
        loop {
            let token =
                self.expect_token(ErrorType::TypeAnnotation, "Expected a type annotation")?;
            match token.value {
                TokenValue::Identifier(_) => {
                    return Ok(TypeAnnot {
                        base: self.parse_base()?,
                        modifiers,
                        span: token.span - start.unwrap().span,
                    });
                }
                TokenValue::Keyword(kw) => {
                    self.advance();
                    modifiers.push(self.parse_type_modifier(kw)?);
                }
                _ => {
                    return Err(self.error(ErrorType::TypeAnnotation, "Expected a type annotation"));
                }
            }
        }
    }

    fn parse_base(&mut self) -> Result<TypeAnnotBase, Error> {
        if !self.is_keyword(TokenType::Fn) {
            return Ok(TypeAnnotBase::Normal(self.parse_name()?));
        }
        self.advance();
        self.expect_keyword(
            TokenType::OpenParen,
            ErrorType::TypeAnnotation,
            "Expected function argument types",
        )?;
        self.advance();
        let mut args = Vec::new();
        while !self.is_keyword(TokenType::CloseParen) {
            args.push(self.parse_type_annotation()?);
            if !self.is_keyword(TokenType::Comma) {
                break;
            }
            self.advance();
        }
        self.expect_keyword(
            TokenType::CloseParen,
            ErrorType::TypeAnnotation,
            "Expected `)`",
        )?;
        self.advance();
        if !self.is_keyword(TokenType::ReturnType) {
            return Ok(TypeAnnotBase::Function(FunctionSig { args, ret: None }));
        }
        self.advance();
        Ok(TypeAnnotBase::Function(FunctionSig {
            args,
            ret: Some(Box::new(self.parse_type_annotation()?)),
        }))
    }

    fn parse_type_modifier(&mut self, keyword: TokenType) -> Result<TypeModifier, Error> {
        Ok(match keyword {
            TokenType::Mul => self.parse_pointer()?,
            TokenType::OpenBrace => self.parse_array_or_slice()?,
            _ => {
                return Err(self.error(ErrorType::TypeAnnotation, "Expected a type annotation"));
            }
        })
    }

    fn parse_pointer(&mut self) -> Result<TypeModifier, Error> {
        let mutable = self.is_mutable()?;
        self.advance();
        Ok(TypeModifier {
            mutable,
            typ: TypeModifierType::Pointer,
        })
    }

    fn parse_array_or_slice(&mut self) -> Result<TypeModifier, Error> {
        let (is_array, array_size) = if let Some(uint) = self.is_uint() {
            self.advance();
            (true, uint)
        } else {
            (false, 0)
        };
        if !self.is_keyword(TokenType::CloseBrace) {
            return Err(self.error(ErrorType::TypeAnnotation, "Expected `]`"));
        }
        self.advance();
        let mutable = self.is_mutable()?;
        self.advance();
        Ok(TypeModifier {
            mutable,
            typ: if is_array {
                TypeModifierType::Array(array_size)
            } else {
                TypeModifierType::Slice
            },
        })
    }
}
