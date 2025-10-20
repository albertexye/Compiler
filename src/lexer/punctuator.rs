use super::*;

impl Lexer {
    pub(super) fn read_punctuator(&mut self) -> Result<TokenValue, Error> {
        let mut node = &*trie::KEYWORD_TREE;
        let mut keyword = None;
        let mut kw_i = 0usize;
        let mut i = 0usize;
        while let Some(ch) = self.input.get(self.index + i)
            && ch.is_ascii_punctuation()
        {
            if let Some(next) = node.children.get(ch) {
                node = next;
                if let Some(kw) = node.keyword {
                    keyword = Some(kw);
                    kw_i = i;
                }
                i += 1;
            } else {
                break;
            }
        }
        if let Some(kw) = keyword {
            self.index += kw_i + 1;
            Ok(TokenValue::Keyword(kw))
        } else {
            Err(self.error(
                ErrorType::UnknownCharacter,
                "Unknown punctuator".to_string(),
            ))
        }
    }
}
