use crate::token::{SymbolId, TokenSpan};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Ast {
    pub(crate) entry: SymbolId,
    pub(crate) modules: HashMap<SymbolId, Module>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Module {
    pub(crate) name: SymbolId,
    pub(crate) files: HashMap<SymbolId, File>, // filename: file
    pub(crate) modules: HashMap<SymbolId, Module>,
    pub(crate) dependencies: HashSet<SymbolId>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct File {
    pub(crate) name: SymbolId,
    pub(crate) module: SymbolId,
    pub(crate) imports: HashSet<SymbolId>,
    pub(crate) globals: HashMap<SymbolId, Scope<Rc<Declaration>>>,
    pub(crate) functions: HashMap<SymbolId, Scope<Rc<Function>>>,
    pub(crate) types: HashMap<SymbolId, Scope<Rc<TypeDef>>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum Visibility {
    Public,
    Private,
    Module,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Scope<T> {
    pub(crate) visibility: Visibility,
    pub(crate) value: T,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct TypeDef {
    pub(crate) name: SymbolId,
    pub(crate) body: TypeDefBody,
    pub(crate) span: TokenSpan,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum TypeDefBody {
    Struct(HashMap<SymbolId, TypeAnnot>),
    Enum(HashMap<SymbolId, u64>),
    Union(HashMap<SymbolId, TypeAnnot>),
    Alias(TypeAnnot),
}

#[derive(Debug, PartialEq, Serialize)]
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
    Function(FunctionSig),
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct FunctionSig {
    pub(crate) args: Vec<TypeAnnot>,
    pub(crate) ret: Option<TypeAnnot>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct TypeAnnot {
    pub(crate) base: Rc<Primitive>,
    pub(crate) modifiers: Vec<TypeModifier>,
    pub(crate) span: TokenSpan,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct TypeModifier {
    pub(crate) mutable: bool,
    pub(crate) typ: TypeModifierType,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum TypeModifierType {
    Pointer,
    Slice,
    Array(u64),
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Enum {
    pub(crate) name: SymbolId,
    pub(crate) fields: HashMap<SymbolId, u64>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Struct {
    pub(crate) name: SymbolId,
    pub(crate) fields: HashMap<SymbolId, TypeAnnot>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Union {
    pub(crate) name: SymbolId,
    pub(crate) fields: HashMap<SymbolId, TypeAnnot>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum Type {
    Primitive(Primitive),
    Enum(Enum),
    Struct(Struct),
    Union(Union),
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Function {
    pub(crate) name: SymbolId,
    pub(crate) arguments: Vec<Rc<FunctionArg>>,
    pub(crate) return_type: Option<TypeAnnot>,
    pub(crate) body: Vec<Statement>,
    pub(crate) span: TokenSpan,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct FunctionArg {
    pub(crate) name: SymbolId,
    pub(crate) typ: TypeAnnot,
    pub(crate) span: TokenSpan,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Declaration {
    pub(crate) name: SymbolId,
    pub(crate) mutable: bool,
    pub(crate) typ: TypeAnnot,
    pub(crate) value: Expression,
    pub(crate) span: TokenSpan,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Expression {
    pub(crate) value: ExpressionValue,
    pub(crate) typ: TypeAnnot,
    pub(crate) span: TokenSpan,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum Identifier {
    Declaraction(Rc<Declaration>),
    Function(Rc<Function>),
    Argument(Rc<FunctionArg>),
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum ExpressionValue {
    Binary(Binary),
    Unary(Unary),
    Call(Call),
    Literal(Literal),
    Identifier(Identifier),
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Binary {
    pub(crate) left: Box<Expression>,
    pub(crate) right: Box<Expression>,
    pub(crate) op: BinaryOp,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum BinaryOp {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    LeftShift,
    RightShift,
    BitAnd,
    BitOr,
    BitXor,
    Gt,
    Ge,
    Lt,
    Le,
    Eq,
    NotEq,
    LogicalAnd,
    LogicalOr,
    Indexing,
    FieldAccess,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Unary {
    pub(crate) operand: Box<Expression>,
    pub(crate) op: UnaryOp,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum UnaryOp {
    LogicalNot,
    BitNot,
    Dereference,
    AddressOf,
    Negate,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Call {
    pub(crate) function: Box<Expression>,
    pub(crate) args: Vec<Expression>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum Literal {
    String(String),
    UInt(u64),
    Int(i64),
    Float(f64),
    Bool(bool),
    Array(Vec<Expression>),
    Struct(HashMap<SymbolId, Expression>),
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Assignment {
    pub(crate) left: Expression,
    pub(crate) right: Expression,
    pub(crate) typ: AssignmentType,
    pub(crate) span: TokenSpan,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum AssignmentType {
    Assign,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    LeftShift,
    RightShift,
    BitAnd,
    BitOr,
    BitXor,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct ConditionalBranch {
    pub(crate) condition: Expression,
    pub(crate) body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Conditional {
    pub(crate) if_branch: ConditionalBranch,
    pub(crate) elif_branches: Vec<ConditionalBranch>,
    pub(crate) else_branch: Option<Vec<Statement>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Match {
    pub(crate) value: Expression,
    pub(crate) cases: Vec<ConditionalBranch>,
    pub(crate) default: Option<Vec<Statement>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Loop {
    pub(crate) init: Option<Rc<Declaration>>,
    pub(crate) condition: Option<Expression>,
    pub(crate) update: Vec<Statement>,
    pub(crate) body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum Statement {
    Declaration(Rc<Declaration>),
    Assignment(Assignment),
    Expression(Expression),
    Loop(Loop),
    Continue(TokenSpan),
    Break(TokenSpan),
    Conditional(Conditional),
    Match(Match),
    Return(Expression),
}
