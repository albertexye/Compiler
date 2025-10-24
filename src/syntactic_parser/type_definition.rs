/*!
This crate parses all kinds of type definitions.
- Struct
- Union
- Enum
- Alias

Structs are of the following format:
```
struct Point {
    x: u64,
    y: u64,
}
```

Unions are of the following format:
```
union Person {
    teacher: Teacher,
    student: Student,
}
```

Enums are of the following format:
```
enum PersonType {
    Teacher,
    Student = 10,
}
```

Aliases are of the following format:
```
use Names = []var []var u8;
```
*/

use std::collections::{HashMap, HashSet};

use super::*;
use syntax_ast::{TypeAnnot, TypeDef, TypeDefBody};

impl SyntacticParser {
    pub(super) fn parse_type_definition(&mut self) -> Result<TypeDef, Error> {
        let token = self.peek().unwrap();
        let TokenValue::Keyword(kw) = token.value else {
            panic!("Type definition starts with a keyword");
        };
        match kw {
            TokenType::Struct => self.parse_struct(),
            TokenType::Enum => self.parse_enum(),
            TokenType::Union => self.parse_union(),
            TokenType::Use => self.parse_alias(),
            _ => panic!("Invalid keyword for type definition"),
        }
    }

    fn parse_struct(&mut self) -> Result<TypeDef, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Struct));
        self.advance();
        let name = self
            .is_identifier()
            .ok_or(self.error(ErrorType::TypeDefinition, "Expected an identifier"))?;
        let span = self.peek().unwrap().span;
        self.advance();
        let fields = self.parse_struct_body()?;
        Ok(TypeDef {
            name,
            body: TypeDefBody::Struct(fields),
            span,
        })
    }

    fn parse_struct_body(&mut self) -> Result<HashMap<SymbolId, TypeAnnot>, Error> {
        if !self.is_keyword(TokenType::OpenBracket) {
            return Err(self.error(ErrorType::TypeDefinition, "Expected `{`"));
        }
        self.advance();
        let mut fields = HashMap::new();
        while !self.is_keyword(TokenType::CloseBracket) {
            let (name, field_type) = self.parse_struct_field()?;
            if fields.contains_key(&name) {
                return Err(self.error(ErrorType::TypeDefinition, "Duplicated struct field "));
            }
            fields.insert(name, field_type);
            if !self.is_keyword(TokenType::Comma) {
                break;
            }
            self.advance();
        }
        self.advance();
        Ok(fields)
    }

    fn parse_struct_field(&mut self) -> Result<(SymbolId, TypeAnnot), Error> {
        let id = self
            .is_identifier()
            .ok_or(self.error(ErrorType::TypeDefinition, "Expected an identifier"))?;
        self.advance();
        if !self.is_keyword(TokenType::Colon) {
            return Err(self.error(
                ErrorType::TypeDefinition,
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
            .ok_or(self.error(ErrorType::TypeDefinition, "Expected an identifier"))?;
        let span = self.peek().unwrap().span;
        self.advance();
        let fields = self.parse_enum_body()?;
        Ok(TypeDef {
            name,
            body: TypeDefBody::Enum(fields),
            span,
        })
    }

    fn parse_enum_body(&mut self) -> Result<HashMap<SymbolId, u64>, Error> {
        if !self.is_keyword(TokenType::OpenBracket) {
            return Err(self.error(ErrorType::TypeDefinition, "Expected `{`"));
        }
        self.advance();
        let mut fields = HashMap::new();
        let mut values = HashSet::new();
        let mut counter: u64 = 0;
        while !self.is_keyword(TokenType::CloseBracket) {
            let (name, value) = self.parse_enum_field(counter)?;
            if fields.contains_key(&name) {
                return Err(self.error(ErrorType::TypeDefinition, "Duplicated enum field"));
            }
            if values.contains(&value) {
                return Err(self.error(ErrorType::TypeDefinition, "Duplicated enum value"));
            }
            fields.insert(name, value);
            values.insert(value);
            counter = value + 1;
            if !self.is_keyword(TokenType::Comma) {
                if self.is_keyword(TokenType::CloseBracket) {
                    break;
                } else {
                    return Err(self.error(ErrorType::TypeDefinition, "Expected `}`"));
                }
            }
            self.advance();
        }
        self.advance();
        Ok(fields)
    }

    fn parse_enum_field(&mut self, counter: u64) -> Result<(SymbolId, u64), Error> {
        let id = self
            .is_identifier()
            .ok_or(self.error(ErrorType::TypeDefinition, "Expected an identifier"))?;
        self.advance();
        if !self.is_keyword(TokenType::Assign) {
            return Ok((id, counter));
        }
        self.advance();
        let value = self.is_uint().ok_or(self.error(
            ErrorType::TypeDefinition,
            "Expected a positive integer value",
        ))?;
        self.advance();
        Ok((id, value))
    }

    fn parse_union(&mut self) -> Result<TypeDef, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Union));
        self.advance();
        let name = match self.is_identifier() {
            Some(name) => name,
            None => {
                return Err(self.error(ErrorType::TypeDefinition, "Expected an identifier"));
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

    fn parse_alias(&mut self) -> Result<TypeDef, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Use));
        self.advance();
        let name = match self.is_identifier() {
            Some(name) => name,
            None => {
                return Err(self.error(ErrorType::TypeDefinition, "Expected an identifier"));
            }
        };
        let span = self.peek().unwrap().span;
        self.advance();
        if !self.is_keyword(TokenType::Eq) {
            return Err(self.error(ErrorType::TypeDefinition, "Expected `=`"));
        }
        self.advance();
        let typ = self.parse_type_annotation()?;
        self.end_line()?;
        Ok(TypeDef {
            name,
            body: TypeDefBody::Alias(typ),
            span,
        })
    }
}
