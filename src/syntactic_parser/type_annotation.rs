use super::*;
use syntax_ast::{TypeAnnot, TypeModifier, TypeModifierType};

impl SyntacticParser {
    pub(crate) fn parse_type_annotation(&mut self) -> Result<TypeAnnot, Error> {
        let mut modifiers = Vec::new();
        let start = self.peek();
        loop {
            let token =
                self.expect_token(ErrorType::TypeAnnotation, "Expected a type annotation")?;
            match token.value {
                TokenValue::Identifier(_) => {
                    return Ok(TypeAnnot {
                        base: self.parse_name()?,
                        modifiers,
                        span: token.span - start.unwrap().span,
                    });
                }
                TokenValue::Keyword(kw) => {
                    modifiers.push(self.parse_type_modifier(kw)?);
                    self.advance();
                }
                _ => {
                    return Err(
                        self.error(ErrorType::TypeAnnotation, "Expected a type annotation")
                    );
                }
            }
        }
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
