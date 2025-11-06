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
        let id = pool.insert(identifier);
        if InternPool::is_keyword(&id) {
            TokenValue::Keyword(InternPool::get_keyword(&id).unwrap())
        } else {
            TokenValue::Identifier(id)
        }
    }
}
