use std::collections::{HashMap, HashSet};

use super::*;
use syntax_ast::{TypeAnnot, TypeDef, TypeDefBody};

impl SyntacticParser {
    pub(crate) fn parse_type_definition(&mut self) -> Result<TypeDef, Error> {
        let token = self.peek().unwrap();
        let TokenValue::Keyword(kw) = token.value else {
            panic!("Type definition starts with a keyword");
        };
        match kw {
            TokenType::Struct => self.parse_struct(),
            TokenType::Enum => self.parse_enum(),
            TokenType::Union => self.parse_union(),
            _ => panic!("Invalid keyword for type definition"),
        }
    }

    fn parse_struct(&mut self) -> Result<TypeDef, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Struct));
        self.advance();
        let name = self
            .is_identifier()
            .ok_or(self.error(&ErrorType::TypeDefinition, "Expected an identifier"))?;
        let span = self.peek().unwrap().span;
        self.advance();
        let fields = self.parse_struct_body()?;
        Ok(TypeDef {
            name,
            body: TypeDefBody::Struct(fields),
            span,
        })
    }

    fn parse_struct_body(&mut self) -> Result<HashMap<String, TypeAnnot>, Error> {
        if !self.is_keyword(TokenType::OpenBracket) {
            return Err(self.error(&ErrorType::TypeDefinition, "Expected `{`"));
        }
        self.advance();
        let mut fields = HashMap::new();
        while !self.is_keyword(TokenType::CloseBracket) {
            let (name, field_type) = self.parse_struct_field()?;
            if fields.contains_key(&name) {
                return Err(self.error(
                    &ErrorType::TypeDefinition,
                    &format!("Duplicated struct field `{}`", name),
                ));
            }
            fields.insert(name, field_type);
            if !self.is_keyword(TokenType::Comma) {
                break;
            }
            self.advance();
        }
        Ok(fields)
    }

    fn parse_struct_field(&mut self) -> Result<(String, TypeAnnot), Error> {
        let id = self
            .is_identifier()
            .ok_or(self.error(&ErrorType::TypeDefinition, "Expected an identifier"))?;
        self.advance();
        if !self.is_keyword(TokenType::Colon) {
            return Err(self.error(
                &ErrorType::TypeDefinition,
                "Expected `:` after an identifier",
            ));
        }
        self.advance();
        let type_annotation = self.parse_type_annotation()?;
        Ok((id, type_annotation))
    }

    fn parse_enum(&mut self) -> Result<TypeDef, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Enum));
        self.advance();
        let name = self
            .is_identifier()
            .ok_or(self.error(&ErrorType::TypeDefinition, "Expected an identifier"))?;
        let span = self.peek().unwrap().span;
        self.advance();
        let fields = self.parse_enum_body()?;
        Ok(TypeDef {
            name,
            body: TypeDefBody::Enum(fields),
            span,
        })
    }

    fn parse_enum_body(&mut self) -> Result<HashMap<String, u64>, Error> {
        if !self.is_keyword(TokenType::OpenBracket) {
            return Err(self.error(&ErrorType::TypeDefinition, "Expected `{`"));
        }
        self.advance();
        let mut fields = HashMap::new();
        let mut values = HashSet::new();
        let mut prev_value: u64 = 0;
        while !self.is_keyword(TokenType::CloseBracket) {
            let (name, value) = self.parse_enum_field(prev_value)?;
            if fields.contains_key(&name) {
                return Err(self.error(
                    &ErrorType::TypeDefinition,
                    &format!("Duplicated enum field `{}`", name),
                ));
            }
            if values.contains(&value) {
                return Err(self.error(
                    &ErrorType::TypeDefinition,
                    &format!("Duplicated enum value `{}`", name),
                ));
            }
            fields.insert(name, value);
            values.insert(value);
            prev_value = value;
            if !self.is_keyword(TokenType::Comma) {
                if self.is_keyword(TokenType::CloseBracket) {
                    break;
                } else {
                    return Err(self.error(&ErrorType::TypeDefinition, "Expected `}`"));
                }
            }
            self.advance();
        }
        self.advance();
        Ok(fields)
    }

    fn parse_enum_field(&mut self, prev: u64) -> Result<(String, u64), Error> {
        let id = self
            .is_identifier()
            .ok_or(self.error(&ErrorType::TypeDefinition, "Expected an identifier"))?;
        self.advance();
        if !self.is_keyword(TokenType::Eq) {
            return Ok((id, prev + 1));
        }
        self.advance();
        let value = self.is_uint().ok_or(self.error(
            &ErrorType::TypeDefinition,
            "Expected a positive integer value",
        ))?;
        Ok((id, value))
    }

    fn parse_union(&mut self) -> Result<TypeDef, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Union));
        self.advance();
        let name = match self.is_identifier() {
            Some(name) => name,
            None => {
                return Err(self.error(&ErrorType::TypeDefinition, "Expected an identifier"));
            }
        };
        let span = self.peek().unwrap().span;
        self.advance();
        let fields = self.parse_struct_body()?;
        Ok(TypeDef {
            name,
            body: TypeDefBody::Union(fields),
            span,
        })
    }
}
