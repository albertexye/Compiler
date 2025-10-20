use super::*;
use syntax_ast::Declaration;

impl SyntacticParser {
    pub(super) fn parse_declaration(&mut self) -> Result<Declaration, Error> {
        let start = self.peek().unwrap().span;
        let mutable = self.is_mutable()?;
        self.advance();
        let id = self.expect_identifier(ErrorType::Declaration, "Expected an identifier")?;
        self.advance();
        if !self.is_keyword(TokenType::Colon) {
            return Err(self.error(ErrorType::Declaration, "Variable type must be specified"));
        }
        self.advance();
        let type_annotation = self.parse_type_annotation()?;
        if !self.is_keyword(TokenType::Eq) {
            return Err(self.error(ErrorType::Declaration, "Variable must be initialized"));
        }
        self.advance();
        let expression = self.parse_expression()?;
        let end = self.peek();
        self.end_line()?;
        let end = end.unwrap().span;
        Ok(Declaration {
            name: id,
            typ: type_annotation,
            value: expression,
            mutable,
            span: end - start,
        })
    }
}
