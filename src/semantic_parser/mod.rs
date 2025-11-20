use crate::intern_pool::SymbolId;
use crate::rw_arc::RwArc;
use crate::semantic_ast::{
    Ast, Declaration, Expression, ExpressionValue, File, Function, FunctionArg, FunctionType,
    Literal, Module, Type, TypeDef, TypeDefBody, TypeId,
};
use crate::span::Span;
use crate::token::TokenType;
use crate::{intern_pool, syntax_ast};
use std::collections::HashMap;
use syntax_ast::Scope;

pub(crate) enum ErrorType {
    Import,
    Type,
}

pub(crate) struct Error {
    pub(crate) typ: ErrorType,
    pub(crate) msg: &'static str,
    pub(crate) span: Span,
}

pub(crate) struct SemanticParser {
    type_id: TypeId,
}

enum TypeColor {
    Unvisited,
    Visiting,
    Visited,
}

impl SemanticParser {
    fn collect_names(&mut self, ast: &syntax_ast::Ast) -> Ast {
        let mut modules = HashMap::new();
        for (module_name, module) in ast.modules.iter() {
            modules.insert(*module_name, self.collect_module_names(module));
        }
        Ast {
            entry: ast.entry,
            modules,
        }
    }

    fn collect_module_names(&mut self, module: &syntax_ast::Module) -> RwArc<Module> {
        let mut submodules = HashMap::new();
        for (submodule_name, submodule) in module.submodules.iter() {
            submodules.insert(*submodule_name, self.collect_module_names(submodule));
        }
        let mut files = HashMap::new();
        for (file_name, file) in module.files.iter() {
            files.insert(*file_name, self.collect_file_names(file));
        }
        RwArc::new(Module {
            name: module.name,
            files,
            submodules,
        })
    }

    fn collect_file_names(&mut self, file: &syntax_ast::File) -> File {
        let mut globals = HashMap::new();
        for (global_name, global) in file.globals.iter() {
            globals.insert(*global_name, Self::build_global_skeleton(global));
        }
        let mut functions = HashMap::new();
        for (function_name, function) in file.functions.iter() {
            functions.insert(*function_name, Self::build_function_skeleton(function));
        }
        let mut types = HashMap::new();
        for (type_name, typ) in file.types.iter() {
            types.insert(*type_name, self.build_type_skeleton(typ));
        }
        File {
            name: file.name,
            module: file.module,
            imports: HashMap::new(),
            globals,
            functions,
            types,
        }
    }

    fn build_global_skeleton(global: &Scope<syntax_ast::Declaration>) -> Scope<RwArc<Declaration>> {
        Scope {
            visibility: global.visibility,
            value: RwArc::new(Declaration {
                name: global.value.name,
                mutable: global.value.mutable,
                typ: Type::U8,
                value: Expression {
                    value: ExpressionValue::Literal(Literal::UInt(0)),
                    typ: Type::U8,
                    span: global.value.value.span,
                },
                span: global.value.span,
            }),
        }
    }

    fn build_function_skeleton(function: &Scope<syntax_ast::Function>) -> Scope<RwArc<Function>> {
        let mut arguments = Vec::new();
        for argument in function.value.arguments.iter() {
            arguments.push(RwArc::new(FunctionArg {
                name: argument.name,
                typ: Type::U8,
                span: argument.span,
            }));
        }
        Scope {
            visibility: function.visibility,
            value: RwArc::new(Function {
                name: function.value.name,
                arguments,
                return_type: None,
                body: Vec::new(),
                span: function.value.span,
            }),
        }
    }

    fn build_type_skeleton(&mut self, typ: &Scope<syntax_ast::TypeDef>) -> Scope<RwArc<TypeDef>> {
        let body = match &typ.value.body {
            syntax_ast::TypeDefBody::Struct(struct_) => {
                let mut fields = HashMap::new();
                for (name, _) in struct_.iter() {
                    fields.insert(*name, Type::U8);
                }
                TypeDefBody::Struct(fields)
            }
            syntax_ast::TypeDefBody::Enum(enum_) => {
                let mut fields = HashMap::new();
                for (name, _) in enum_.iter() {
                    fields.insert(*name, 0);
                }
                TypeDefBody::Enum(fields)
            }
            syntax_ast::TypeDefBody::Union(union) => {
                let mut fields = HashMap::new();
                for (name, _) in union.iter() {
                    fields.insert(*name, Type::U8);
                }
                TypeDefBody::Union(fields)
            }
            syntax_ast::TypeDefBody::Alias(_) => TypeDefBody::Alias(Type::U8),
        };
        let id = self.type_id;
        self.type_id.0 += 1;
        Scope {
            visibility: typ.visibility,
            value: RwArc::new(TypeDef {
                id,
                name: typ.value.name,
                body,
                size: 0,
                span: typ.value.span,
            }),
        }
    }
}

fn resolve_module_deps(syn_module: &syntax_ast::Module, sem_ast: &mut Ast) -> Result<(), Error> {
    for dep in syn_module.dependencies.iter() {
        if !sem_ast.modules.contains_key(dep) {
            return Err(Error {
                typ: ErrorType::Import,
                msg: "Dependency doesn't exist",
                span: Span::path_only(syn_module.path),
            });
        }
    }
    Ok(())
}

fn resolve_file_imports(
    syn_module: &syntax_ast::Module,
    syn_file: &syntax_ast::File,
    sem_file: &mut File,
    sem_ast: &Ast,
) -> Result<(), Error> {
    for (import, span) in syn_file.imports.iter() {
        if !syn_module.dependencies.contains(import) {
            return Err(Error {
                typ: ErrorType::Import,
                msg: "Importing undeclared module",
                span: *span,
            });
        }
        let imported = sem_ast.modules.get(import).unwrap();
        sem_file.imports.insert(*import, (imported).clone());
    }
    Ok(())
}

fn resolve_func_sig(sem_file: &File, sig: &syntax_ast::FunctionSig) -> Result<Type, Error> {
    let mut args = Vec::new();
    for arg in sig.args.iter() {
        args.push(resolve_type_annot(sem_file, arg)?);
    }
    let ret = match &sig.ret {
        Some(ret) => Some(Box::new(resolve_type_annot(sem_file, &ret)?)),
        None => None,
    };
    Ok(Type::Function(FunctionType { args, ret }))
}

fn keyword_to_primitive(kwd: TokenType) -> Option<Type> {
    // I'm really sinful for designing such a bad thing.
    // Repent!
    // Maybe I should use a macro...
    Some(match kwd {
        TokenType::U8 => Type::U8,
        TokenType::U16 => Type::U16,
        TokenType::U32 => Type::U32,
        TokenType::U64 => Type::U64,
        TokenType::Usize => Type::Usize,
        TokenType::I8 => Type::I8,
        TokenType::I16 => Type::I16,
        TokenType::I32 => Type::I32,
        TokenType::I64 => Type::I64,
        TokenType::Isize => Type::Isize,
        TokenType::F32 => Type::F32,
        TokenType::F64 => Type::F64,
        TokenType::Bool => Type::Bool,
        _ => return None,
    })
}

fn resolve_immediate_type(sem_file: &File, type_name: SymbolId, span: Span) -> Result<Type, Error> {
    let ret = if intern_pool::is_keyword(&type_name) {
        keyword_to_primitive(intern_pool::get_keyword(&type_name))
    } else {
        match sem_file.types.get(&type_name) {
            Some(typ) => Some(Type::Custom(typ.value.clone())),
            None => None,
        }
    };
    match ret {
        Some(typ) => Ok(typ),
        None => Err(Error {
            typ: ErrorType::Type,
            msg: "Can't resolve type name",
            span,
        }),
    }
}

fn resolve_type_annot(sem_file: &File, type_annot: &syntax_ast::TypeAnnot) -> Result<Type, Error> {
    todo!("also search the submodules!");
    todo!("scopes matters!");
    let name = match &type_annot.base {
        syntax_ast::TypeAnnotBase::Normal(name) => name,
        syntax_ast::TypeAnnotBase::Function(sig) => return resolve_func_sig(sem_file, sig),
    };
    let ret = 'block: {
        if name.len() == 2 {
            break 'block None;
        }
        if name.len() == 1 {
            break 'block Some(resolve_immediate_type(sem_file, name[0], type_annot.span)?);
        }
        let mut module = match sem_file.imports.get(&name[0]) {
            Some(module) => module.clone(),
            None => break 'block None,
        };
        for module_name in &name[1..name.len() - 1] {
            // This trick makes sure the module is not being borrowed and reassigned at the same time.
            let tmp_module = module.clone();
            let guard = tmp_module.read().unwrap();
            module = match guard.submodules.get(module_name) {
                Some(module) => module.clone(),
                None => break 'block None,
            };
        }
        let guard = module.read().unwrap();
        let file = match guard.files.get(&name[name.len() - 2]) {
            Some(file) => file,
            None => break 'block None,
        };
        match file.types.get(&name[name.len() - 1]) {
            Some(typ) => Some(Type::Custom(typ.value.clone())),
            None => None,
        }
    };
    match ret {
        Some(typ) => Ok(typ),
        None => Err(Error {
            typ: ErrorType::Type,
            msg: "Can't resolve type name",
            span: type_annot.span,
        }),
    }
}

fn resolve_file_types(syn_file: &syntax_ast::File, sem_file: &mut File) -> Result<(), Error> {}

fn resolve_type(
    sem_file: &File,
    syn_typ: syntax_ast::TypeDef,
    sem_typ: RwArc<TypeDef>,
    type_status: HashMap<TypeId, TypeColor>,
) -> Result<(), Error> {
    let mut guard = sem_typ.write().unwrap();
    match syn_typ.body {
        syntax_ast::TypeDefBody::Enum(_) => {
            guard.size = 8; // size_of(u64) is very meaningless
        }
        syntax_ast::TypeDefBody::Alias(alias) => {
            guard.body = TypeDefBody::Alias(resolve_type_annot(sem_file, &alias)?);
        }
        syntax_ast::TypeDefBody::Struct(fields) => {
            for (field_name, type_annot) in fields.iter() {
                let typ = sem_file.types.get(field_name).unwrap();
            }
        }
        syntax_ast::TypeDefBody::Union(fields) => {}
    }
    Ok(())
}
