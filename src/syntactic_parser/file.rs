use crate::syntax_ast::Visibility;

use super::*;
use syntax_ast::{File, Scope};

impl SyntacticParser {
    pub(crate) fn parse_file(&mut self, filename: &str) -> Result<File, Error> {
        let module = self.parse_module()?;
        let imports = self.parse_imports()?;
        let mut globals = Vec::new();
        let mut functions = Vec::new();
        let mut types = Vec::new();
        while self.peek().is_some() {
            let visibility = self.parse_visibility()?;
            let token = self.expect_token(&ErrorType::Module, "Missing symbol definition")?;
            let TokenValue::Keyword(kw) = token.value else {
                return Err(self.error(&ErrorType::Module, "Expected keyword"));
            };
            match kw {
                TokenType::Struct | TokenType::Enum | TokenType::Union | TokenType::Use => {
                    types.push(Scope {
                        visibility,
                        value: self.parse_type_definition()?,
                    });
                }
                TokenType::Let | TokenType::Var => {
                    globals.push(Scope {
                        visibility,
                        value: self.parse_declaration()?,
                    });
                }
                TokenType::Fn => {
                    functions.push(Scope {
                        visibility,
                        value: self.parse_function()?,
                    });
                }
                _ => {
                    return Err(self.error(&ErrorType::Module, "Invalid top level definition"));
                }
            }
        }
        Ok(File {
            name: filename.to_string(),
            module,
            imports,
            globals,
            functions,
            types,
        })
    }

    fn parse_visibility(&mut self) -> Result<Visibility, Error> {
        if self.is_keyword(TokenType::Pub) {
            self.advance();
            Ok(Visibility::Public)
        } else if self.is_keyword(TokenType::Prv) {
            self.advance();
            Ok(Visibility::Private)
        } else if self.is_keyword(TokenType::Mod) {
            self.advance();
            Ok(Visibility::Module)
        } else {
            Err(self.error(&ErrorType::Module, "Expected visibility specifier"))
        }
    }

    fn parse_module(&mut self) -> Result<String, Error> {
        if !self.is_keyword(TokenType::Module) {
            return Err(self.error(
                &ErrorType::Module,
                "A file must start with a module declaration",
            ));
        }
        self.advance();
        let name = self.is_identifier().ok_or(self.error(
            &ErrorType::Module,
            "Keyword `module` must be followed by a valid identifier",
        ))?;
        self.advance();
        Ok(name)
    }

    fn parse_imports(&mut self) -> Result<Vec<String>, Error> {
        let mut imports = Vec::new();
        while self.is_keyword(TokenType::Import) {
            imports.push(self.parse_import()?);
        }
        Ok(imports)
    }

    fn parse_import(&mut self) -> Result<String, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Import));
        self.advance();
        let name = self.is_identifier().ok_or(self.error(
            &ErrorType::Import,
            "Keyword `import` must be followed by a valid identifier",
        ))?;
        self.advance();
        self.end_line()?;
        Ok(name)
    }
}
