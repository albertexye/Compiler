use crate::token::*;

mod identifier;
mod number;
mod punctuator;
mod skip;
mod string;
mod utils;

pub(crate) struct Lexer {
    input: Vec<char>,

    index: usize,
    line: usize,
    column: usize,

    start_index: usize,
    start_line: usize,
    start_column: usize,

    pub(crate) symbol_table: SymbolTable,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum ErrorType {
    UnclosedString,
    InvalidEscapeSequence,
    InvalidNumber,
    UnknownCharacter,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Error {
    typ: ErrorType,
    span: TokenSpan,
    msg: &'static str,
}

impl Lexer {
    pub(crate) fn new() -> Self {
        Lexer {
            input: Vec::new(),
            index: 0,
            line: 1,
            column: 1,
            start_index: 0,
            start_line: 1,
            start_column: 1,
            symbol_table: SymbolTable::new(),
        }
    }

    pub(crate) fn lex(&mut self, input: &str) -> Result<Vec<Token>, Error> {
        self.input = input.chars().collect();
        self.index = 0;
        self.line = 1;
        self.column = 1;
        self.start_index = 0;
        self.start_line = 1;
        self.start_column = 1;
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        self.input = Vec::new();
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Literal, Token, TokenSpan, TokenValue};

    // Helper to create a TokenSpan
    fn span(line: usize, column: usize, index: usize, size: usize) -> TokenSpan {
        TokenSpan {
            line,
            column,
            index,
            size,
        }
    }

    // Helper function to compare lexed tokens with expected tokens, including spans.
    fn assert_lexes(input: &str, expected: Vec<Token>) {
        let mut lexer = Lexer::new();
        let tokens = lexer.lex(input).unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_empty_input() {
        assert_lexes("", vec![]);
    }

    #[test]
    fn test_whitespace_and_comments() {
        assert_lexes("  // this is a comment\n  ", vec![]);
        assert_lexes("// another comment", vec![]);
    }

    #[test]
    fn test_integers() {
        assert_lexes(
            "123",
            vec![Token {
                value: TokenValue::Literal(Literal::UInt(123)),
                span: span(1, 1, 0, 3),
            }],
        );
        assert_lexes(
            "-45",
            vec![Token {
                value: TokenValue::Literal(Literal::Int(-45)),
                span: span(1, 1, 0, 3),
            }],
        );
    }

    #[test]
    fn test_hexadecimal_numbers() {
        assert_lexes(
            "0x1A",
            vec![Token {
                value: TokenValue::Literal(Literal::UInt(26)),
                span: span(1, 1, 0, 4),
            }],
        );
        assert_lexes(
            "0Xff",
            vec![Token {
                value: TokenValue::Literal(Literal::UInt(255)),
                span: span(1, 1, 0, 4),
            }],
        );
    }

    #[test]
    fn test_binary_numbers() {
        assert_lexes(
            "0b1010",
            vec![Token {
                value: TokenValue::Literal(Literal::UInt(10)),
                span: span(1, 1, 0, 6),
            }],
        );
    }

    #[test]
    fn test_float_numbers() {
        assert_lexes(
            "123.456",
            vec![Token {
                value: TokenValue::Literal(Literal::Float(123.456)),
                span: span(1, 1, 0, 7),
            }],
        );
        assert_lexes(
            "-0.5",
            vec![Token {
                value: TokenValue::Literal(Literal::Float(-0.5)),
                span: span(1, 1, 0, 4),
            }],
        );
    }

    #[test]
    fn test_number_errors() {
        let mut lexer = Lexer::new();
        assert!(lexer.lex("0xG").is_err());
        assert!(lexer.lex("0b2").is_err());
    }

    #[test]
    fn test_string_literals() {
        assert_lexes(
            r#""hello""#,
            vec![Token {
                value: TokenValue::Literal(Literal::String("hello".to_string())),
                span: span(1, 1, 0, 7),
            }],
        );
        assert_lexes(
            r#""escaped \" \n \t \\""#,
            vec![Token {
                value: TokenValue::Literal(Literal::String("escaped \" \n \t \\".to_string())),
                span: span(1, 1, 0, 21),
            }],
        );
    }

    #[test]
    fn test_unclosed_string() {
        let mut lexer = Lexer::new();
        assert!(lexer.lex(r#""hello"#).is_err());
    }

    #[test]
    fn test_identifiers_and_keywords() {
        let mut lexer = Lexer::new();
        let tokens = lexer.lex("let x = 5;").unwrap();
        let expected = vec![
            Token {
                value: TokenValue::Keyword(TokenType::Let),
                span: span(1, 1, 0, 3),
            },
            Token {
                value: TokenValue::Identifier(lexer.symbol_table.search("x").unwrap()),
                span: span(1, 5, 4, 1),
            },
            Token {
                value: TokenValue::Keyword(TokenType::Assign),
                span: span(1, 7, 6, 1),
            },
            Token {
                value: TokenValue::Literal(Literal::UInt(5)),
                span: span(1, 9, 8, 1),
            },
            Token {
                value: TokenValue::Keyword(TokenType::Semicolon),
                span: span(1, 10, 9, 1),
            },
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_punctuators() {
        assert_lexes(
            "+ - * /",
            vec![
                Token {
                    value: TokenValue::Keyword(TokenType::Plus),
                    span: span(1, 1, 0, 1),
                },
                Token {
                    value: TokenValue::Keyword(TokenType::Minus),
                    span: span(1, 3, 2, 1),
                },
                Token {
                    value: TokenValue::Keyword(TokenType::Mul),
                    span: span(1, 5, 4, 1),
                },
                Token {
                    value: TokenValue::Keyword(TokenType::Div),
                    span: span(1, 7, 6, 1),
                },
            ],
        );
    }

    #[test]
    fn test_multiline_lexing() {
        let mut lexer = Lexer::new();
        let tokens = lexer.lex("let y\n  = 10;").unwrap();
        let expected = vec![
            Token {
                value: TokenValue::Keyword(TokenType::Let),
                span: span(1, 1, 0, 3),
            },
            Token {
                value: TokenValue::Identifier(lexer.symbol_table.search("y").unwrap()),
                span: span(1, 5, 4, 1),
            },
            Token {
                value: TokenValue::Keyword(TokenType::Assign),
                span: span(2, 3, 8, 1),
            },
            Token {
                value: TokenValue::Literal(Literal::UInt(10)),
                span: span(2, 5, 10, 2),
            },
            Token {
                value: TokenValue::Keyword(TokenType::Semicolon),
                span: span(2, 7, 12, 1),
            },
        ];
        assert_eq!(tokens, expected);
    }
}
