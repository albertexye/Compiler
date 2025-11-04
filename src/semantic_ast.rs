use std::collections::HashMap;
use std::rc::Rc;

use crate::token::SymbolId;

pub(crate) enum Primitive {
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
    Bool,
    Slice(Rc<Type>),
    Array(Rc<Type>, usize),
    Pointer(Rc<Type>),
}

pub(crate) struct Enum {
    pub(crate) name: SymbolId,
    pub(crate) fields: HashMap<SymbolId, u64>,
}

pub(crate) struct Struct {
    pub(crate) name: SymbolId,
    pub(crate) fields: HashMap<SymbolId, Rc<Type>>,
}

pub(crate) struct Union {
    pub(crate) name: SymbolId,
    pub(crate) fields: HashMap<SymbolId, Rc<Type>>,
}

pub(crate) enum Type {
    Primitive(Primitive),
    Enum(Enum),
    Struct(Struct),
    Union(Union),
}

pub(crate) struct TypeTree {
    typ: Rc<Type>,
    children: HashMap<SymbolId, TypeTree>,
}
