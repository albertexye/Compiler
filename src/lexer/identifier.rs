use super::*;

impl Lexer {
    pub(super) fn read_identifier(&mut self) -> TokenValue {
        let mut id = Vec::new();
        while let Some(&ch) = self.peek()
            && (ch.is_alphanumeric() || ch == '_')
        {
            id.push(ch);
            self.advance();
        }
        if let Some(kw) = trie::search_token(&id) {
            TokenValue::Keyword(kw)
        } else {
            TokenValue::Identifier(id.iter().collect())
        }
    }
}
