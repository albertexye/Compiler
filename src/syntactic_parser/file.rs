use std::collections::{HashMap, HashSet};

use crate::syntax_ast::{Declaration, Function, TypeDef, Visibility};

use super::*;
use syntax_ast::{File, Scope};

impl SyntacticParser {
    pub(super) fn parse_file(&mut self, filename: &str, module_name: &str) -> Result<File, Error> {
        let module = self.parse_module_declaration()?;
        if module != module_name {
            return Err(self.error(ErrorType::Module, "Incorrect module name"));
        }
        let imports = self.parse_imports()?;
        let mut types = HashMap::new();
        let mut globals = HashMap::new();
        let mut functions = HashMap::new();
        while self.peek().is_some() {
            self.parse_content(&mut types, &mut globals, &mut functions)?;
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

    fn parse_content(
        &mut self,
        types: &mut HashMap<String, Scope<TypeDef>>,
        globals: &mut HashMap<String, Scope<Declaration>>,
        functions: &mut HashMap<String, Scope<Function>>,
    ) -> Result<(), Error> {
        let visibility = self.parse_visibility()?;
        let token = self.expect_token(ErrorType::Module, "Missing symbol definition")?;
        let TokenValue::Keyword(kw) = token.value else {
            return Err(self.error(ErrorType::Module, "Expected keyword"));
        };
        match kw {
            TokenType::Struct | TokenType::Enum | TokenType::Union | TokenType::Use => {
                let value = self.parse_type_definition()?;
                if types
                    .insert(value.name.clone(), Scope { visibility, value })
                    .is_some()
                {
                    return Err(self.error(ErrorType::Module, "Duplicated type name"));
                }
            }
            TokenType::Let | TokenType::Var => {
                let value = self.parse_declaration()?;
                if globals
                    .insert(value.name.clone(), Scope { visibility, value })
                    .is_some()
                {
                    return Err(self.error(ErrorType::Module, "Duplicated global name"));
                }
            }
            TokenType::Fn => {
                let value = self.parse_function()?;
                if functions
                    .insert(value.name.clone(), Scope { visibility, value })
                    .is_some()
                {
                    return Err(self.error(ErrorType::Module, "Duplicated function name"));
                }
            }
            _ => {
                return Err(self.error(ErrorType::Module, "Invalid top level definition"));
            }
        }
        Ok(())
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
            Err(self.error(ErrorType::Module, "Expected visibility specifier"))
        }
    }

    fn parse_module_declaration(&mut self) -> Result<String, Error> {
        if !self.is_keyword(TokenType::Module) {
            return Err(self.error(
                ErrorType::Module,
                "A file must start with a module declaration",
            ));
        }
        self.advance();
        let name = self.is_identifier().ok_or(self.error(
            ErrorType::Module,
            "Keyword `module` must be followed by a valid identifier",
        ))?;
        self.advance();
        Ok(name)
    }

    fn parse_imports(&mut self) -> Result<HashSet<String>, Error> {
        let mut imports = HashSet::new();
        while self.is_keyword(TokenType::Import) {
            if !imports.insert(self.parse_import()?) {
                return Err(self.error(ErrorType::Import, "Duplicated imports"));
            }
        }
        Ok(imports)
    }

    fn parse_import(&mut self) -> Result<String, Error> {
        std::debug_assert!(self.is_keyword(TokenType::Import));
        self.advance();
        let name = self.is_identifier().ok_or(self.error(
            ErrorType::Import,
            "Keyword `import` must be followed by a valid identifier",
        ))?;
        self.advance();
        self.end_line()?;
        Ok(name)
    }
}
