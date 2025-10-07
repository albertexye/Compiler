use super::*;

impl Lexer {
    pub(crate) fn skip_whitespace_and_comments(&mut self) {
        while self.skip_whitespace() || self.skip_comment() {}
    }

    fn skip_whitespace(&mut self) -> bool {
        let mut found = false;
        while let Some(&ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            found = true;
            self.advance();
        }
        found
    }

    fn skip_comment(&mut self) -> bool {
        if self.peek() == Some(&'/') && self.peek2() == Some(&'/') {
            while let Some(&ch) = self.peek() {
                self.advance();
                if ch == '\n' {
                    break;
                }
            }
            true
        } else {
            false
        }
    }
}
