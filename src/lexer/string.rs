use super::*;

impl Lexer {
    /// Reads a string literal token.
    pub(super) fn read_string(&mut self) -> Result<TokenValue, Error> {
        debug_assert_eq!(self.peek(), Some(&'"'));
        self.advance(); // skip opening quote
        let mut string_content = String::new();
        while let Some(&ch) = self.peek() {
            if ch == '\\' {
                self.advance();
                string_content.push(self.read_escape_sequence()?);
                continue;
            }
            if ch == '"' {
                self.advance();
                return Ok(self.make_string(string_content));
            }
            if ch == '\n' {
                return Err(self.unclosed_string_error());
            }
            if ch.is_control() {
                return Err(self.control_char_in_string_error(ch));
            }
            string_content.push(ch);
            self.advance();
        }
        Err(self.unclosed_string_error())
    }

    fn make_string(&self, content: String) -> TokenValue {
        TokenValue::Literal(Literal::String(content))
    }

    fn unclosed_string_error(&self) -> Error {
        self.error(
            ErrorType::UnclosedString,
            "Unclosed string literal".to_string(),
        )
    }

    fn control_char_in_string_error(&self, ch: char) -> Error {
        self.error(
            ErrorType::InvalidEscapeSequence,
            format!("Control character in string literal: {}", ch),
        )
    }

    fn read_escape_sequence(&mut self) -> Result<char, Error> {
        let ch = match self.peek() {
            Some(&ch) => ch,
            None => {
                return Err(self.error(
                    ErrorType::InvalidEscapeSequence,
                    "No character after \\".to_string(),
                ));
            }
        };
        self.advance();
        match ch {
            'n' => Ok('\n'),
            't' => Ok('\t'),
            'r' => Ok('\r'),
            '\\' => Ok('\\'),
            '"' => Ok('"'),
            'x' => self.read_hexidecimal_escape_sequence(),
            'u' => self.read_unicode_escape_sequence(),
            _ => Err(self.error(
                ErrorType::InvalidEscapeSequence,
                format!("Invalid escape sequence: \\{}", ch),
            )),
        }
    }

    fn read_hexidecimal_escape_sequence(&mut self) -> Result<char, Error> {
        let h1 = self.peek();
        let h2 = self.peek2();
        if let (Some(&h1), Some(&h2)) = (h1, h2) {
            let hex_str = format!("{}{}", h1, h2);
            if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                self.advance();
                self.advance();
                Ok(byte as char)
            } else {
                Err(self.error(
                    ErrorType::InvalidEscapeSequence,
                    format!("Invalid hex escape sequence: \\x{}", hex_str),
                ))
            }
        } else {
            Err(self.error(
                ErrorType::InvalidEscapeSequence,
                "Incomplete hex escape sequence".to_string(),
            ))
        }
    }

    fn read_unicode_escape_sequence(&mut self) -> Result<char, Error> {
        if self.peek() != Some(&'{') {
            return Err(self.error(
                ErrorType::InvalidEscapeSequence,
                "Expected '{' after \\u".to_string(),
            ));
        }
        self.advance();
        while let Some(&ch) = self.peek() {
            if ch == '}' {
                break;
            }
            if !ch.is_ascii_hexdigit() {
                return Err(self.error(
                    ErrorType::InvalidEscapeSequence,
                    format!("Invalid character in Unicode escape: {}", ch),
                ));
            }
            self.advance();
        }
        if self.peek() != Some(&'}') {
            return Err(self.error(
                ErrorType::InvalidEscapeSequence,
                "Unclosed Unicode escape sequence".to_string(),
            ));
        }
        let hex_str: String = self.input[self.start_index + 2..self.index]
            .iter()
            .collect();
        self.advance();
        if let Ok(code_point) = u32::from_str_radix(&hex_str, 16) {
            if let Some(ch) = std::char::from_u32(code_point) {
                Ok(ch)
            } else {
                Err(self.error(
                    ErrorType::InvalidEscapeSequence,
                    format!("Invalid Unicode code point: \\u{{{}}}", hex_str),
                ))
            }
        } else {
            Err(self.error(
                ErrorType::InvalidEscapeSequence,
                format!("Invalid Unicode escape sequence: \\u{{{}}}", hex_str),
            ))
        }
    }
}
