use super::*;

impl Lexer {
    pub(super) fn read_identifier(&mut self, pool: &mut InternPool) -> TokenValue {
        let mut identifier = Vec::new();
        while let Some(&ch) = self.peek()
            && (ch.is_alphanumeric() || ch == '_')
        {
            identifier.push(ch);
            self.advance();
        }
        let identifier = identifier.iter().collect();
        let id = pool.insert_symbol(identifier);
        if intern_pool::is_keyword(&id) {
            TokenValue::Keyword(intern_pool::get_keyword(&id))
        } else {
            TokenValue::Identifier(id)
        }
    }
}
