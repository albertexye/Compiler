use crate::intern_pool::SymbolId;
use crate::rw_arc::RwArc;
use crate::span::Span;
use crate::syntax_ast::Scope;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Ast {
    pub(crate) entry: SymbolId,
    pub(crate) modules: HashMap<SymbolId, RwArc<Module>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Module {
    pub(crate) name: SymbolId,
    pub(crate) files: HashMap<SymbolId, File>, // filename: file
    pub(crate) submodules: HashMap<SymbolId, RwArc<Module>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct File {
    pub(crate) name: SymbolId,
    pub(crate) module: SymbolId,
    pub(crate) imports: HashMap<SymbolId, RwArc<Module>>,
    pub(crate) globals: HashMap<SymbolId, Scope<RwArc<Declaration>>>,
    pub(crate) functions: HashMap<SymbolId, Scope<RwArc<Function>>>,
    pub(crate) types: HashMap<SymbolId, Scope<RwArc<TypeDef>>>,
}

#[derive(Debug, PartialEq, Serialize, Clone, Copy)]
pub(crate) struct TypeId(pub(crate) usize);

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct TypeDef {
    pub(crate) id: TypeId,
    pub(crate) name: SymbolId,
    pub(crate) body: TypeDefBody,
    pub(crate) size: usize,
    pub(crate) span: Span,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum TypeDefBody {
    Struct(HashMap<SymbolId, Type>),
    Enum(HashMap<SymbolId, u64>),
    Union(HashMap<SymbolId, Type>),
    Alias(Type),
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum Type {
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

    Custom(RwArc<TypeDef>),

    Function(FunctionType),

    Pointer {
        inner: Box<Type>,
        mutable: bool,
    },
    Slice {
        inner: Box<Type>,
        mutable: bool,
    },
    Array {
        inner: Box<Type>,
        size: u64,
        mutable: bool,
    },
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct FunctionType {
    pub(crate) args: Vec<Type>,
    pub(crate) ret: Option<Box<Type>>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Function {
    pub(crate) name: SymbolId,
    pub(crate) arguments: Vec<RwArc<FunctionArg>>,
    pub(crate) return_type: Option<Type>,
    pub(crate) body: Vec<Statement>,
    pub(crate) span: Span,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct FunctionArg {
    pub(crate) name: SymbolId,
    pub(crate) typ: Type,
    pub(crate) span: Span,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Declaration {
    pub(crate) name: SymbolId,
    pub(crate) mutable: bool,
    pub(crate) typ: Type,
    pub(crate) value: Expression,
    pub(crate) span: Span,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) struct Expression {
    pub(crate) value: ExpressionValue,
    pub(crate) typ: Type,
    pub(crate) span: Span,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum Identifier {
    Declaraction(RwArc<Declaration>),
    Function(RwArc<Function>),
    Argument(RwArc<FunctionArg>),
    EnumVariant(RwArc<TypeDef>, SymbolId),
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
    pub(crate) span: Span,
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
    pub(crate) init: Option<RwArc<Declaration>>,
    pub(crate) condition: Option<Expression>,
    pub(crate) update: Vec<Statement>,
    pub(crate) body: Vec<Statement>,
}

#[derive(Debug, PartialEq, Serialize)]
pub(crate) enum Statement {
    Declaration(RwArc<Declaration>),
    Assignment(Assignment),
    Expression(Expression),
    Loop(Loop),
    Continue(Span),
    Break(Span),
    Conditional(Conditional),
    Match(Match),
    Return(Expression),
}
