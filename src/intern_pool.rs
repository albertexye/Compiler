use crate::token::{TOKEN_TYPES_ENUM, TOKEN_TYPES_STR, TokenType};
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

/// The RefCell is only used to store thread-local serialization
///     contexts. This serialization only happens in test builds.
#[cfg(test)]
use std::cell::RefCell;

/// SymbolId holds the id of a unique identifier or punctuator.
/// Serialization is automatically implemented, but a custom
///     version is specifically defined for test builds.
/// Therefore, Serialize is not derived for test builds.
#[cfg_attr(not(test), derive(Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SymbolId(usize);

/// PathId holds the id of a unique PathBuf.
/// Serialization is automatically implemented, but a custom
///     version is specifically defined for test builds.
/// Therefore, Serialize is not derived for test builds.
#[cfg_attr(not(test), derive(Serialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PathId(usize);

/// Since the usize fields of PathId is private, a PathId is predefined
///     for tests, as tests often use only one file, and the id is going
///     to be 0.
#[cfg(test)]
pub(crate) const TEST_PATH_ID: PathId = PathId(0);

/// The InternPool interns symbols and paths.
pub(crate) struct InternPool {
    /// The next symbol id value. Increments when a new symbol is inserted.
    symbol_counter: SymbolId,
    /// Stores all unique symbols. This field is no longer valid after the first
    ///     reverse lookup is performed. The time the first reverse lookup is performed
    ///     is the time the program is about to exit due to errors. Therefore, it's
    ///     safe to assume no additional symbols are going to be inserted.
    symbol_pool: HashMap<String, SymbolId>,
    /// The reversed id to symbol lookup array. The value is usually None. When the
    ///     first reverse lookup is performed, the entire symbol_pool is mapped to
    ///     this array for lookup. Values are moved so symbol_pool becomes invalid.
    symbol_reverse: Option<Vec<String>>,

    /// The following 3 fields serve the same purpose, but for paths.
    path_counter: PathId,
    path_pool: HashMap<PathBuf, PathId>,
    path_reverse: Option<Vec<PathBuf>>,
}

/// The keyword part of the symbol table is constructed using the constants defined in
///     token.rs, and the ids for them are deterministic. Specifically, their index is
///     their id.
pub(crate) fn is_keyword(id: &SymbolId) -> bool {
    id.0 < TOKEN_TYPES_STR.len()
}

/// Get the SymbolId for a keyword. Panics if keyword is not a keyword.
/// This function should have been made const, but due to the limitations
///     of the current Rust version, Rust can't safely make this const.
pub(crate) fn get_keyword_symbol_id(keyword: &'static str) -> SymbolId {
    SymbolId(TOKEN_TYPES_STR.iter().position(|&x| x == keyword).unwrap())
}

/// Get the TokenType enum value of a keyword. Panics if the id
///     does not point to a keyword.
pub(crate) fn get_keyword(id: &SymbolId) -> TokenType {
    if !is_keyword(id) {
        panic!("Not a keyword");
    } else {
        TOKEN_TYPES_ENUM[id.0]
    }
}

impl InternPool {
    /// Create an InternPool with keywords built in.
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

    /// Inserts the token into the pool and returns the SymbolId.
    /// If the token exists, the existing SymbolId is returned.
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

    /// Inserts the path into the pool and returns the PathId.
    /// If the path exists, the existing PathId is returned.
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

    /// If the token exists, returns the SymbolId; otherwise, returns None.
    pub(crate) fn search_symbol(&self, token: &str) -> Option<SymbolId> {
        std::debug_assert!(self.symbol_reverse.is_none());
        if self.symbol_pool.contains_key(token) {
            Some(self.symbol_pool[token])
        } else {
            None
        }
    }

    /// Reverses the pools if it's not already done.
    /// After this, nothing can be inserted or searched,
    ///     only reverse conversions are allowed.
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

    /// Gets the string value of a SymbolId. After the first call to this function,
    ///     nothing can be inserted or searched anymore.
    pub(crate) fn symbol_reverse_lookup(&mut self, id: SymbolId) -> Option<String> {
        self.reverse();
        let rev = self.symbol_reverse.as_ref().unwrap();
        if id.0 < rev.len() {
            Some(rev[id.0].clone())
        } else {
            None
        }
    }

    /// Gets the path value of a PathId. After the first call to this function,
    ///     nothing can be inserted or searched anymore.
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
    /// The SYMBOL_CONTEXT holds the InternPool for reverse lookup.
    /// This reason this global variable exists is because serde won't
    ///     allow context to be passed to the serialization functions.
    /// However, snapshot testing (which is a very viable way to test ASTs)
    ///     requires everything to be serialized.
    /// Ids are not understandable to humans when printed out, so we want a
    ///     way to see what the true value the id has.
    /// So a global context is used to do exactly that.
    /// This is a heck. Since Rust runs tests in parallel, each thread can run
    ///     at most one test at a time. So we reserve the context for each thread.
    /// This eliminates the need to use a lock and makes tests run faster.
    /// RefCell is used so that the context can be switched.
    /// Of course, this variable only exists for test builds.
    static SYMBOL_CONTEXT: RefCell<InternPool> = RefCell::new(InternPool::new());
}

/// Sets the symbol context of THIS THREAD.
/// The whole symbol context idea is only available in test builds.
#[cfg(test)]
pub(crate) fn set_symbol_context(pool: InternPool) {
    SYMBOL_CONTEXT.with(|c| {
        *c.borrow_mut() = pool;
    });
}

/// The test-only serialization method for SymbolIds.
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

/// The test-only serialization method for PathIds.
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
