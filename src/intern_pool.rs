use crate::token::{TOKEN_TYPES_ENUM, TOKEN_TYPES_STR, TokenType};
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

#[cfg(test)]
use std::cell::RefCell;

#[cfg_attr(not(test), derive(Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SymbolId(usize);

#[cfg_attr(not(test), derive(Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PathId(usize);

#[cfg(test)]
pub(crate) const TEST_PATH_ID: PathId = PathId(0);

pub(crate) struct InternPool {
    symbol_counter: SymbolId,
    symbol_pool: HashMap<String, SymbolId>,
    symbol_reverse: Option<Vec<String>>,

    path_counter: PathId,
    path_pool: HashMap<PathBuf, PathId>,
    path_reverse: Option<Vec<PathBuf>>,
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
            symbol_counter: SymbolId(0),
            symbol_pool: HashMap::new(),
            symbol_reverse: None,

            path_counter: PathId(0),
            path_pool: HashMap::new(),
            path_reverse: None,
        };
        for keyword in TOKEN_TYPES_STR {
            pool.symbol_pool
                .insert(keyword.to_string(), pool.symbol_counter);
            pool.symbol_counter.0 += 1;
        }
        pool
    }

    pub(crate) fn insert_symbol(&mut self, token: String) -> SymbolId {
        std::debug_assert!(self.symbol_reverse.is_none());
        if self.symbol_pool.contains_key(&token) {
            self.symbol_pool[&token]
        } else {
            let id = self.symbol_counter;
            self.symbol_pool.insert(token, self.symbol_counter);
            self.symbol_counter.0 += 1;
            id
        }
    }

    pub(crate) fn insert_path(&mut self, path: PathBuf) -> PathId {
        std::debug_assert!(self.path_reverse.is_none());
        if self.path_pool.contains_key(&path) {
            self.path_pool[&path]
        } else {
            let id = self.path_counter;
            self.path_pool.insert(path, self.path_counter);
            self.path_counter.0 += 1;
            id
        }
    }

    pub(crate) fn search_symbol(&self, token: &str) -> Option<SymbolId> {
        std::debug_assert!(self.symbol_reverse.is_none());
        if self.symbol_pool.contains_key(token) {
            Some(self.symbol_pool[token])
        } else {
            None
        }
    }

    fn reverse(&mut self) {
        if self.symbol_reverse.is_none() {
            let mut reverse = vec![String::new(); self.symbol_counter.0];
            let pool = std::mem::take(&mut self.symbol_pool);
            for (sym, id) in pool.into_iter() {
                reverse[id.0] = sym;
            }
            self.symbol_reverse = Some(reverse);
        }
        if self.path_reverse.is_none() {
            let mut reverse = vec![PathBuf::new(); self.path_counter.0];
            let pool = std::mem::take(&mut self.path_pool);
            for (path, id) in pool.into_iter() {
                reverse[id.0] = path;
            }
            self.path_reverse = Some(reverse);
        }
    }

    pub(crate) fn symbol_reverse_lookup(&mut self, id: SymbolId) -> Option<String> {
        self.reverse();
        let rev = self.symbol_reverse.as_ref().unwrap();
        if id.0 < rev.len() {
            Some(rev[id.0].clone())
        } else {
            None
        }
    }

    pub(crate) fn path_reverse_lookup(&mut self, id: PathId) -> Option<PathBuf> {
        self.reverse();
        let rev = self.path_reverse.as_ref().unwrap();
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
        let token = SYMBOL_CONTEXT.with(|c| c.borrow_mut().symbol_reverse_lookup(*self));
        serializer.serialize_str(&token.unwrap())
    }
}

#[cfg(test)]
impl Serialize for PathId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let path = SYMBOL_CONTEXT.with(|c| c.borrow_mut().path_reverse_lookup(*self));
        serializer.serialize_str(path.unwrap().to_str().unwrap())
    }
}
