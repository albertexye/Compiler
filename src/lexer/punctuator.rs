use super::*;

impl Lexer {
    pub(super) fn read_punctuator(&mut self) -> Result<TokenValue, Error> {
        let mut punc = Vec::new();
        let mut keyword = None;
        let mut kw_i = 0usize;
        let mut i = 0usize;
        while let Some(ch) = self.input.get(self.index + i)
            && ch.is_ascii_punctuation()
        {
            punc.push(*ch);
            let s: String = punc.iter().collect();
            if let Some(id) = self.symbol_table.search(&s) {
                if SymbolTable::is_keyword(&id) {
                    keyword = SymbolTable::get_keyword(&id);
                    kw_i = i;
                }
                i += 1;
            } else {
                break;
            }
        }
        if let Some(kw) = keyword {
            self.index += kw_i + 1;
            self.column += kw_i + 1;
            Ok(TokenValue::Keyword(kw))
        } else {
            Err(self.error(ErrorType::UnknownCharacter, "Unknown punctuator"))
        }
    }
}
