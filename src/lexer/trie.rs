use super::*;

use std::collections::HashMap;
use std::sync::LazyLock;

const TOKEN_TYPES_STR: [&str; 63] = [
    ",", ";", ":", "::", ".", "(", ")", "[", "]", "{", "}", "+", "+=", "-", "-=", "*", "*=", "/",
    "/=", "%", "%=", "<<", "<<=", ">>", ">>=", "&", "&=", "|", "|=", "^", "^=", "~", "and", "or",
    "!", "==", "!=", ">", ">=", "<", "<=", "=", "->", "=>", "if", "else", "match", "while", "for",
    "break", "continue", "return", "fn", "let", "var", "struct", "enum", "union", "pub", "prv",
    "module", "import", "use",
];

const TOKEN_TYPES_ENUM: [TokenType; 63] = [
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
    TokenType::Mod,
    TokenType::ModEq,
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
    TokenType::Module,
    TokenType::Import,
    TokenType::Use,
];

const _: () = std::assert!(
    TOKEN_TYPES_STR.len() == TOKEN_TYPES_ENUM.len(),
    "Mismatched token type definition"
);

#[derive(Default, Debug)]
pub(crate) struct TokenTypeNode {
    pub(crate) character: char,
    pub(crate) keyword: Option<TokenType>,
    pub(crate) children: HashMap<char, TokenTypeNode>,
}

fn build_token_type_tree() -> TokenTypeNode {
    let mut root = TokenTypeNode::default();
    for (i, word) in TOKEN_TYPES_STR.iter().enumerate() {
        let mut node = &mut root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_default();
        }
        node.keyword = Some(TOKEN_TYPES_ENUM[i]);
    }
    root
}

pub(crate) fn search_token(word: &[char]) -> Option<TokenType> {
    let mut node = &*KEYWORD_TREE;
    for ch in word {
        if let Some(next) = node.children.get(ch) {
            node = next;
        } else {
            return None;
        }
    }
    node.keyword
}

pub(crate) static KEYWORD_TREE: LazyLock<TokenTypeNode> = LazyLock::new(|| build_token_type_tree());

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(TOKEN_TYPES_STR[*self as usize])
    }
}
