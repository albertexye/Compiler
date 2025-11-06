use serde::Serialize;
#[cfg(test)]
use std::cell::RefCell;
use std::collections::HashMap;
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
    Identifier(SymbolId),
    Literal(Literal),
    Keyword(TokenType),
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
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

const TOKEN_TYPES_STR: [&str; 64] = [
    ",", ";", ":", "::", ".", "(", ")", "[", "]", "{", "}", "+", "+=", "-", "-=", "*", "*=", "/",
    "/=", "%", "%=", "<<", "<<=", ">>", ">>=", "&", "&=", "|", "|=", "^", "^=", "~", "and", "or",
    "!", "==", "!=", ">", ">=", "<", "<=", "=", "->", "=>", "if", "else", "match", "while", "for",
    "break", "continue", "return", "fn", "let", "var", "struct", "enum", "union", "pub", "prv",
    "mod", "module", "import", "use",
];

const TOKEN_TYPES_ENUM: [TokenType; 64] = [
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
];

const _: () = assert!(TOKEN_TYPES_STR.len() == TOKEN_TYPES_ENUM.len());

#[cfg_attr(not(test), derive(Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SymbolId(usize);

pub(crate) struct InternPool {
    counter: SymbolId,
    pool: HashMap<String, SymbolId>,
    reverse: Option<Vec<String>>,
}

impl InternPool {
    pub(crate) fn new() -> InternPool {
        let mut pool = InternPool {
            counter: SymbolId(0),
            pool: HashMap::new(),
            reverse: None,
        };
        for keyword in TOKEN_TYPES_STR {
            pool.pool.insert(keyword.to_string(), pool.counter);
            pool.counter.0 += 1;
        }
        pool
    }

    pub(crate) fn insert(&mut self, token: String) -> SymbolId {
        std::debug_assert!(self.reverse.is_none());
        if self.pool.contains_key(&token) {
            self.pool[&token]
        } else {
            let id = self.counter;
            self.pool.insert(token, self.counter);
            self.counter.0 += 1;
            id
        }
    }

    pub(crate) fn search(&self, token: &str) -> Option<SymbolId> {
        std::debug_assert!(self.reverse.is_none());
        if self.pool.contains_key(token) {
            Some(self.pool[token])
        } else {
            None
        }
    }

    pub(crate) fn is_keyword(id: &SymbolId) -> bool {
        id.0 < TOKEN_TYPES_STR.len()
    }

    pub(crate) fn get_keyword(id: &SymbolId) -> Option<TokenType> {
        if !InternPool::is_keyword(id) {
            None
        } else {
            Some(TOKEN_TYPES_ENUM[id.0])
        }
    }

    pub(crate) fn reverse_lookup(&mut self, id: SymbolId) -> Option<String> {
        let rev = match self.reverse.as_ref() {
            Some(rev) => rev,
            None => {
                let mut reverse = vec![String::new(); self.counter.0];
                let pool = std::mem::take(&mut self.pool);
                for (sym, id) in pool.into_iter() {
                    reverse[id.0] = sym;
                }
                self.reverse = Some(reverse);
                self.reverse.as_ref().unwrap()
            }
        };
        if id.0 < rev.len() {
            Some(rev[id.0].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
thread_local! {
    static SYMBOL_CONTEXT: RefCell<InternPool> = RefCell::new(InternPool::new());
}

#[cfg(test)]
pub(crate) fn set_symbol_context(pool: InternPool) {
    SYMBOL_CONTEXT.with(|c| {
        *c.borrow_mut() = pool;
    });
}

#[cfg(test)]
impl Serialize for SymbolId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let token = SYMBOL_CONTEXT.with(|c| c.borrow_mut().reverse_lookup(*self));
        serializer.serialize_str(&token.unwrap())
    }
}
