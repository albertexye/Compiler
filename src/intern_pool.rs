use crate::token::{TOKEN_TYPES_ENUM, TOKEN_TYPES_STR, TokenType};
use serde::Serialize;
use std::collections::HashMap;

#[cfg(test)]
use std::cell::RefCell;

#[cfg_attr(not(test), derive(Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SymbolId(usize);

pub(crate) struct InternPool {
    counter: SymbolId,
    pool: HashMap<String, SymbolId>,
    reverse: Option<Vec<String>>,
}

pub(crate) fn is_keyword(id: &SymbolId) -> bool {
    id.0 < TOKEN_TYPES_STR.len()
}

pub(crate) fn get_keyword_symbol_id(keyword: &'static str) -> SymbolId {
    SymbolId(TOKEN_TYPES_STR.iter().position(|&x| x == keyword).unwrap())
}

pub(crate) fn get_keyword(id: &SymbolId) -> TokenType {
    if !is_keyword(id) {
        panic!("Not a keyword");
    } else {
        TOKEN_TYPES_ENUM[id.0]
    }
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
