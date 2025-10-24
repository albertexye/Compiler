use serde::Serialize;
use std::collections::HashMap;
use std::ops::Sub;
#[cfg(test)]
use std::sync::{LazyLock, Mutex};

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

pub(crate) struct SymbolTable {
    counter: SymbolId,
    table: HashMap<String, SymbolId>,
    reverse: Option<Vec<String>>,
}

impl SymbolTable {
    pub(crate) fn new() -> SymbolTable {
        let mut sym_table = SymbolTable {
            counter: SymbolId(0),
            table: HashMap::new(),
            reverse: None,
        };
        for keyword in TOKEN_TYPES_STR {
            sym_table
                .table
                .insert(keyword.to_string(), sym_table.counter);
            sym_table.counter.0 += 1;
        }
        sym_table
    }

    pub(crate) fn insert(&mut self, token: String) -> SymbolId {
        std::debug_assert!(self.reverse.is_none());
        if self.table.contains_key(&token) {
            self.table[&token]
        } else {
            let id = self.counter;
            self.table.insert(token, self.counter);
            self.counter.0 += 1;
            id
        }
    }

    pub(crate) fn search(&self, token: &str) -> Option<SymbolId> {
        std::debug_assert!(self.reverse.is_none());
        if self.table.contains_key(token) {
            Some(self.table[token])
        } else {
            None
        }
    }

    pub(crate) fn is_keyword(id: &SymbolId) -> bool {
        id.0 < TOKEN_TYPES_STR.len()
    }

    pub(crate) fn get_keyword(id: &SymbolId) -> Option<TokenType> {
        if !SymbolTable::is_keyword(id) {
            None
        } else {
            Some(TOKEN_TYPES_ENUM[id.0])
        }
    }

    pub(crate) fn reverse_lookup(&mut self, id: SymbolId) -> String {
        let rev = match self.reverse.as_ref() {
            Some(rev) => rev,
            None => {
                let mut reverse = vec![String::new(); self.counter.0];
                let table = std::mem::take(&mut self.table);
                for (sym, id) in table.into_iter() {
                    reverse[id.0] = sym;
                }
                self.reverse = Some(reverse);
                self.reverse.as_ref().unwrap()
            }
        };
        assert!(id.0 < rev.len());
        rev[id.0].clone()
    }
}

#[cfg(test)]
static SYMBOL_CONTEXT: LazyLock<Mutex<SymbolTable>> =
    LazyLock::new(|| Mutex::new(SymbolTable::new()));

#[cfg(test)]
pub(crate) fn set_symbol_context(symbol_table: SymbolTable) {
    let mut guard = SYMBOL_CONTEXT.lock().unwrap();
    *guard = symbol_table;
}

#[cfg(test)]
impl Serialize for SymbolId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut guard = SYMBOL_CONTEXT.lock().unwrap();
        serializer.serialize_str(&guard.reverse_lookup(*self))
    }
}
