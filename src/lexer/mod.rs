//! The lexer module turns the input file content into tokens.
//! The token values are interned to improve performance.
//! Each token also stores where it's from, including the file
//!     path (interned) and the exact span in the file.

use crate::intern_pool;
use crate::intern_pool::{InternPool, PathId};
use crate::span::Span;
use crate::token::{Literal, Token, TokenValue};

mod identifier;
mod number;
mod punctuator;
mod skip;
mod string;
mod utils;

/// The Lexer object, one for a file.
/// This struct only holds the state of the Lexer, not the result.
/// So it can be considered as a intermediate construct.
/// There's no reason to ever construct Lexer, since Lexer::lex
///     is what you need.
pub(crate) struct Lexer {
    /// Which file we are lexing
    path: PathId,
    /// The input String gets turned into a Vec of char for easier processing
    input: Vec<char>,

    /// Current index in the original text, counted in characters.
    /// This index points to the next char to be processed.
    index: usize,
    /// Current line number, only used to generate Span
    line: usize,
    /// Current column number, only used to generate Span
    column: usize,

    /// These 3 fields serve the same purpose as the above 3,
    /// except they are pointing to the beginning to the token
    /// being processed. This makes it easier to track the span
    /// of a token.
    start_index: usize,
    start_line: usize,
    start_column: usize,
}

/// Lexer error types
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum ErrorType {
    /// A string that's missing a `"`.
    UnclosedString,
    /// Invalid Unicode escape sequence in a string.
    InvalidEscapeSequence,
    /// Invalid number due to many possible reasons.
    /// 1. Invalid base: currently only bases 2, 10, and 16 are supported.
    ///    So, only 0b, 0x, and normal digits are supported.
    ///    Base 8 isn't supported due to its uselessness.
    /// 2. Integer overflow: If a u64 can't hold a positive number,
    ///    or an i64 can't hold a negative number, an overflow is encountered.
    ///    There's no plan to support integers larger than 64 bits.
    /// 3. No digits after base: If `0x` or `0b` are not immediately followed by one
    ///    or more digits, this error occurs.
    /// 4. No digits after decimal point: If a decimal point is not immediately
    ///    followed by one or more digits, this error occurs. Some languages
    ///    support number literals such as `3.`, but this is generally not
    ///    obvious that it's a floating point number.
    /// 5. Invalid digits: If a number contains digits that don't belong to the base,
    ///    this error occurs. For example, `0b123` is an invalid number.
    InvalidNumber,
    /// An unrecognized character is encountered. The compiler only accepts ASCII
    ///     characters unless the characters are in a string or comment.
    ///     It's generally not good to use Unicode characters to name things,
    ///     as many characters look similar or the same and there are invisible ones.
    UnknownCharacter,
}

/// Lexer error struct
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Error {
    /// The general type of the error.
    typ: ErrorType,
    /// The place the error occurred.
    span: Span,
    /// A description to the error. Note how it's mandated that the message must be a
    ///     static string. Since we already have the error location, there's no need
    ///     to say what part of the text is wrong. This design choice makes the compiler
    ///     faster by avoiding dynamic allocations on errors, though it's not a huge gain
    ///     at all since errors are rare in general.
    msg: &'static str,
}

impl Lexer {
    /// Lex the given file content. The InternPool is shared within the whole compilation
    ///     process, so it's passed to the function.
    pub(crate) fn lex(
        path: PathId,
        input: &str,
        pool: &mut InternPool,
    ) -> Result<Vec<Token>, Error> {
        let mut lexer = Self {
            path,
            input: input.chars().collect(),
            index: 0,
            line: 1,
            column: 1,
            start_index: 0,
            start_line: 1,
            start_column: 1,
        };
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token(pool)? {
            tokens.push(token);
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intern_pool::TEST_PATH_ID;
    use crate::token::TokenType;
    use std::path::PathBuf;

    fn span(line: usize, column: usize, index: usize, size: usize) -> Span {
        Span {
            path: TEST_PATH_ID,
            line,
            column,
            index,
            size,
        }
    }

    fn assert_lexes(input: &str, expected: Vec<Token>) {
        let mut pool = InternPool::new();
        let path_id = pool.insert_path(PathBuf::new());
        let tokens = Lexer::lex(path_id, input, &mut pool).unwrap();
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
        let mut pool = InternPool::new();
        let path_id = pool.insert_path(PathBuf::new());
        assert!(Lexer::lex(path_id, "0xG", &mut pool).is_err());
        assert!(Lexer::lex(path_id, "0b2", &mut pool).is_err());
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
        let mut pool = InternPool::new();
        let path_id = pool.insert_path(PathBuf::new());
        assert!(Lexer::lex(path_id, r#""hello"#, &mut pool).is_err());
    }

    #[test]
    fn test_identifiers_and_keywords() {
        let mut pool = InternPool::new();
        let path_id = pool.insert_path(PathBuf::new());
        let tokens = Lexer::lex(path_id, "let x = 5;", &mut pool).unwrap();
        let expected = vec![
            Token {
                value: TokenValue::Keyword(TokenType::Let),
                span: span(1, 1, 0, 3),
            },
            Token {
                value: TokenValue::Identifier(pool.search_symbol("x").unwrap()),
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
        let mut pool = InternPool::new();
        let path_id = pool.insert_path(PathBuf::new());
        let tokens = Lexer::lex(path_id, "let y\n  = 10;", &mut pool).unwrap();
        let expected = vec![
            Token {
                value: TokenValue::Keyword(TokenType::Let),
                span: span(1, 1, 0, 3),
            },
            Token {
                value: TokenValue::Identifier(pool.search_symbol("y").unwrap()),
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
