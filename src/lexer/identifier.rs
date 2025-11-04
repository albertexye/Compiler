use super::*;

impl Lexer {
    pub(super) fn read_identifier(&mut self, symbol_table: &mut SymbolTable) -> TokenValue {
        let mut identifier = Vec::new();
        while let Some(&ch) = self.peek()
            && (ch.is_alphanumeric() || ch == '_')
        {
            identifier.push(ch);
            self.advance();
        }
        let identifier = identifier.iter().collect();
        let id = symbol_table.insert(identifier);
        if SymbolTable::is_keyword(&id) {
            TokenValue::Keyword(SymbolTable::get_keyword(&id).unwrap())
        } else {
            TokenValue::Identifier(id)
        }
    }
}
