use serde::Serialize;
use std::ops::Sub;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum TokenType {
    Comma,
    Semicolon,
    Colon,
    DoubleColon,
    Dot,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Plus,
    PlusEq,
    Minus,
    MinusEq,
    Mul,
    MulEq,
    Div,
    DivEq,
    Modulo,
    ModuloEq,
    LeftShift,
    LeftShiftEq,
    RightShift,
    RightShiftEq,
    BitAnd,
    BitAndEq,
    BitOr,
    BitOrEq,
    BitXor,
    BitXorEq,
    BitNot,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    Eq,
    NotEq,
    Gt,
    Ge,
    Lt,
    Le,
    Assign,
    ReturnType,
    MatchCase,
    If,
    Else,
    Match,
    While,
    For,
    Break,
    Continue,
    Return,
    Fn,
    Let,
    Var,
    Struct,
    Enum,
    Union,
    Pub,
    Prv,
    Mod,
    Module,
    Import,
    Use,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Literal {
    UInt(u64),
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenValue {
    Identifier(String),
    Literal(Literal),
    Keyword(TokenType),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub(crate) struct TokenSpan {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) index: usize,
    pub(crate) size: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
    pub(crate) value: TokenValue,
    pub(crate) span: TokenSpan,
}

impl Sub for TokenSpan {
    type Output = TokenSpan;

    fn sub(self, other: TokenSpan) -> TokenSpan {
        std::debug_assert!(self.index + self.size >= other.index);
        TokenSpan {
            line: self.line,
            column: self.column,
            index: self.index,
            size: self.index + self.size - other.index,
        }
    }
}
