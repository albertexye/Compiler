use super::*;

impl Lexer {
    fn start_token(&mut self) {
        self.start_index = self.index;
        self.start_line = self.line;
        self.start_column = self.column;
    }

    fn end_token(&self) -> TokenSpan {
        TokenSpan {
            line: self.start_line,
            column: self.start_column,
            index: self.start_index,
            size: self.index - self.start_index,
        }
    }

    pub(super) fn peek(&self) -> Option<&char> {
        self.input.get(self.index)
    }

    pub(super) fn peek2(&self) -> Option<&char> {
        self.input.get(self.index + 1)
    }

    pub(super) fn next_token(&mut self) -> Result<Option<Token>, Error> {
        self.skip_whitespace_and_comments();
        self.start_token();
        if self.peek().is_none() {
            return Ok(None);
        }
        let value = self.next_token_value()?;
        Ok(Some(Token {
            value,
            span: self.end_token(),
        }))
    }

    /// Returns the next token from the input, or None if at end.
    fn next_token_value(&mut self) -> Result<TokenValue, Error> {
        let ch = *self.peek().unwrap();
        if ch.is_alphabetic() || ch == '_' {
            return Ok(self.read_identifier());
        }
        if ch == '"' {
            return self.read_string();
        }
        if ch.is_ascii_punctuation() {
            // Check for negative number: '-' followed by digit
            if ch == '-'
                && let Some(next_ch) = self.peek2()
                && next_ch.is_ascii_digit()
            {
                return self.read_number();
            }
            return self.read_punctuator();
        }
        if ch.is_ascii_digit() {
            return self.read_number();
        }
        Err(self.error(
            ErrorType::UnknownCharacter,
            format!("Unrecognized character: {}", ch),
        ))
    }

    pub(super) fn error(&self, error_type: ErrorType, message: String) -> Error {
        Error {
            error_type,
            span: self.end_token(),
            message,
        }
    }

    pub(super) fn advance(&mut self) {
        if let Some(&ch) = self.peek() {
            self.index += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
}
