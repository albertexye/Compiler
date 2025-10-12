use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use crate::token::TokenSpan;

pub(crate) struct Ast {
    pub(crate) entry: PathBuf,
    pub(crate) modules: HashMap<PathBuf, Module>,
}

pub(crate) struct Module {
    pub(crate) name: String,
    pub(crate) files: HashMap<String, File>,
}

pub(crate) struct File {
    pub(crate) name: String,
    pub(crate) module: String,
    pub(crate) imports: HashSet<String>,
    pub(crate) globals: HashMap<String, Scope<Declaration>>,
    pub(crate) functions: HashMap<String, Scope<Function>>,
    pub(crate) types: HashMap<String, Scope<TypeDef>>,
}

pub(crate) type Name = Vec<String>;

pub(crate) enum Visibility {
    Public,
    Private,
    Module,
}

pub(crate) struct Scope<T> {
    pub(crate) visibility: Visibility,
    pub(crate) value: T,
}

pub(crate) struct TypeDef {
    pub(crate) name: String,
    pub(crate) body: TypeDefBody,
    pub(crate) span: TokenSpan,
}

pub(crate) enum TypeDefBody {
    Struct(HashMap<String, TypeAnnot>),
    Enum(HashMap<String, u64>),
    Union(HashMap<String, TypeAnnot>),
    Alias(TypeAnnot),
}

pub(crate) enum Statement {
    Declaration(Declaration),
    Assignment(Assignment),
    Expression(Expression),
    Loop(Loop),
    Continue(TokenSpan),
    Break(TokenSpan),
    Conditional(Conditional),
    Match(Match),
    Return(Expression),
}

pub(crate) struct TypeAnnot {
    pub(crate) base: Name,
    pub(crate) modifiers: Vec<TypeModifier>,
    pub(crate) span: TokenSpan,
}

pub(crate) struct TypeModifier {
    pub(crate) mutable: bool,
    pub(crate) typ: TypeModifierType,
}

pub(crate) enum TypeModifierType {
    Pointer,
    Slice,
    Array(u64),
}

pub(crate) struct Function {
    pub(crate) name: String,
    pub(crate) arguments: Vec<FunctionArg>,
    pub(crate) return_type: Option<TypeAnnot>,
    pub(crate) body: Vec<Statement>,
    pub(crate) span: TokenSpan,
}

pub(crate) struct FunctionArg {
    pub(crate) name: String,
    pub(crate) typ: TypeAnnot,
    pub(crate) span: TokenSpan,
}

pub(crate) struct Declaration {
    pub(crate) name: String,
    pub(crate) mutable: bool,
    pub(crate) typ: TypeAnnot,
    pub(crate) value: Expression,
    pub(crate) span: TokenSpan,
}

pub(crate) struct Expression {
    pub(crate) value: ExpressionValue,
    pub(crate) span: TokenSpan,
}

pub(crate) enum ExpressionValue {
    Binary(Binary),
    Unary(Unary),
    Call(Call),
    Literal(Literal),
    Identifier(Name),
}

pub(crate) struct Binary {
    pub(crate) left: Box<Expression>,
    pub(crate) right: Box<Expression>,
    pub(crate) op: BinaryOp,
}

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

pub(crate) struct Unary {
    pub(crate) operand: Box<Expression>,
    pub(crate) op: UnaryOp,
}

pub(crate) enum UnaryOp {
    LogicalNot,
    BitNot,
    Dereference,
    AddressOf,
    Negate,
}

pub(crate) struct Call {
    pub(crate) function: Box<Expression>,
    pub(crate) args: Vec<Expression>,
}

pub(crate) enum Literal {
    String(String),
    UInt(u64),
    Int(i64),
    Float(f64),
    Bool(bool),
    Array(Vec<Expression>),
    Struct(HashMap<String, Expression>),
}

pub(crate) struct Assignment {
    pub(crate) left: Expression,
    pub(crate) right: Expression,
    pub(crate) typ: AssignmentType,
    pub(crate) span: TokenSpan,
}

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

pub(crate) struct ConditionalBranch {
    pub(crate) condition: Expression,
    pub(crate) body: Vec<Statement>,
}

pub(crate) struct Conditional {
    pub(crate) if_branch: ConditionalBranch,
    pub(crate) elif_branches: Vec<ConditionalBranch>,
    pub(crate) else_branch: Option<Vec<Statement>>,
}

pub(crate) struct Match {
    pub(crate) value: Expression,
    pub(crate) cases: Vec<ConditionalBranch>,
    pub(crate) default: Option<Vec<Statement>>,
}

pub(crate) struct Loop {
    pub(crate) init: Option<Declaration>,
    pub(crate) condition: Option<Expression>,
    pub(crate) update: Vec<Statement>,
    pub(crate) body: Vec<Statement>,
}
