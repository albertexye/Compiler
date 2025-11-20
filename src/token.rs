/// This file defines Token.
use crate::intern_pool::SymbolId;
use crate::span::Span;

/// A list of builtin keywords or punctuators.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum TokenType {
    // Punctuators
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

    // Keywords
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

    // Literals
    True,
    False,

    // Primitives
    U8,
    U16,
    U32,
    U64,
    I8,
    Usize,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
    Bool,
}

/// Literal values.
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Literal {
    /// All non-negative integer literals are treated as UInt.
    UInt(u64),
    /// Only negative integer literals are treated as Int.
    Int(i64),
    Float(f64),
    String(String),
}

/// Possible token values.
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenValue {
    /// A name. Any word that's not a keyword.
    Identifier(SymbolId),
    /// Any literal values except struct literals and array literals,
    ///     as they are made of other tokens.
    Literal(Literal),
    /// A keyword or a punctuator. They are treated the same at this stage.
    Keyword(TokenType),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Token {
    pub(crate) value: TokenValue,
    pub(crate) span: Span,
}

/// This maps each TokenType with its string representation. It's also used to construct
///     the InternPool.
pub(crate) const TOKEN_TYPES_STR: [&str; 79] = [
    // Punctuators
    ",", ";", ":", "::", ".", "(", ")", "[", "]", "{", "}", "+", "+=", "-", "-=", "*", "*=", "/",
    "/=", "%", "%=", "<<", "<<=", ">>", ">>=", "&", "&=", "|", "|=", "^", "^=", "~", "and", "or",
    "!", "==", "!=", ">", ">=", "<", "<=", "=", "->", "=>", // Keywords
    "if", "else", "match", "while", "for", "break", "continue", "return", "fn", "let", "var",
    "struct", "enum", "union", "pub", "prv", "mod", "module", "import", "use",
    // Literals
    "true", "false", // Primitives
    "u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize", "f32", "f64", "bool",
];

/// Rust doesn't trust programmers to convert an integer back to an enum.
/// Therefore, all of the enum values here are listed in the order they
///     appear in TOKEN_TYPES_STR to perform 2-way conversions.
pub(crate) const TOKEN_TYPES_ENUM: [TokenType; 79] = [
    // Punctuators
    TokenType::Comma,
    TokenType::Semicolon,
    TokenType::Colon,
    TokenType::DoubleColon,
    TokenType::Dot,
    TokenType::OpenParen,
    TokenType::CloseParen,
    TokenType::OpenBrace,
    TokenType::CloseBrace,
    TokenType::OpenBracket,
    TokenType::CloseBracket,
    TokenType::Plus,
    TokenType::PlusEq,
    TokenType::Minus,
    TokenType::MinusEq,
    TokenType::Mul,
    TokenType::MulEq,
    TokenType::Div,
    TokenType::DivEq,
    TokenType::Modulo,
    TokenType::ModuloEq,
    TokenType::LeftShift,
    TokenType::LeftShiftEq,
    TokenType::RightShift,
    TokenType::RightShiftEq,
    TokenType::BitAnd,
    TokenType::BitAndEq,
    TokenType::BitOr,
    TokenType::BitOrEq,
    TokenType::BitXor,
    TokenType::BitXorEq,
    TokenType::BitNot,
    TokenType::LogicalAnd,
    TokenType::LogicalOr,
    TokenType::LogicalNot,
    TokenType::Eq,
    TokenType::NotEq,
    TokenType::Gt,
    TokenType::Ge,
    TokenType::Lt,
    TokenType::Le,
    TokenType::Assign,
    TokenType::ReturnType,
    TokenType::MatchCase,
    // Keywords
    TokenType::If,
    TokenType::Else,
    TokenType::Match,
    TokenType::While,
    TokenType::For,
    TokenType::Break,
    TokenType::Continue,
    TokenType::Return,
    TokenType::Fn,
    TokenType::Let,
    TokenType::Var,
    TokenType::Struct,
    TokenType::Enum,
    TokenType::Union,
    TokenType::Pub,
    TokenType::Prv,
    TokenType::Mod,
    TokenType::Module,
    TokenType::Import,
    TokenType::Use,
    // Literals
    TokenType::True,
    TokenType::False,
    // Primitives
    TokenType::U8,
    TokenType::U16,
    TokenType::U32,
    TokenType::U64,
    TokenType::Usize,
    TokenType::I8,
    TokenType::I16,
    TokenType::I32,
    TokenType::I64,
    TokenType::Isize,
    TokenType::F32,
    TokenType::F64,
    TokenType::Bool,
];

/// A sanity check. They must have the same length.
const _: () = assert!(TOKEN_TYPES_STR.len() == TOKEN_TYPES_ENUM.len());
