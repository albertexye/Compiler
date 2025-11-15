use crate::semantic_ast::{
    Ast, Declaration, Expression, ExpressionValue, File, Function, FunctionArg, Literal, Module,
    Type, TypeDef, TypeDefBody,
};
use crate::span::Span;
use crate::syntax_ast;
use std::collections::HashMap;
use std::rc::Rc;
use syntax_ast::Scope;

pub(crate) enum ErrorType {
    Import,
}

pub(crate) struct Error {
    pub(crate) typ: ErrorType,
    pub(crate) msg: &'static str,
    pub(crate) span: Span,
}

fn collect_names(ast: &syntax_ast::Ast) -> Ast {
    let mut modules = HashMap::new();
    for (module_name, module) in ast.modules.iter() {
        modules.insert(*module_name, collect_module_names(module));
    }
    Ast {
        entry: ast.entry,
        modules,
    }
}

fn collect_module_names(module: &syntax_ast::Module) -> Rc<Module> {
    let mut submodules = HashMap::new();
    for (submodule_name, submodule) in module.submodules.iter() {
        submodules.insert(*submodule_name, collect_module_names(submodule));
    }
    let mut files = HashMap::new();
    for (file_name, file) in module.files.iter() {
        files.insert(*file_name, collect_file_names(file));
    }
    Rc::new(Module {
        name: module.name,
        files,
        submodules,
    })
}

fn build_global_skeleton(global: &Scope<syntax_ast::Declaration>) -> Scope<Rc<Declaration>> {
    Scope {
        visibility: global.visibility,
        value: Rc::new(Declaration {
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

fn build_function_skeleton(function: &Scope<syntax_ast::Function>) -> Scope<Rc<Function>> {
    let mut arguments = Vec::new();
    for argument in function.value.arguments.iter() {
        arguments.push(Rc::new(FunctionArg {
            name: argument.name,
            typ: Type::U8,
            span: argument.span,
        }));
    }
    Scope {
        visibility: function.visibility,
        value: Rc::new(Function {
            name: function.value.name,
            arguments,
            return_type: None,
            body: Vec::new(),
            span: function.value.span,
        }),
    }
}

fn build_type_skeleton(typ: &Scope<syntax_ast::TypeDef>) -> Scope<Rc<TypeDef>> {
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
    Scope {
        visibility: typ.visibility,
        value: Rc::new(TypeDef {
            name: typ.value.name,
            body,
            span: typ.value.span,
        }),
    }
}

fn collect_file_names(file: &syntax_ast::File) -> File {
    let mut globals = HashMap::new();
    for (global_name, global) in file.globals.iter() {
        globals.insert(*global_name, build_global_skeleton(global));
    }
    let mut functions = HashMap::new();
    for (function_name, function) in file.functions.iter() {
        functions.insert(*function_name, build_function_skeleton(function));
    }
    let mut types = HashMap::new();
    for (type_name, typ) in file.types.iter() {
        types.insert(*type_name, build_type_skeleton(typ));
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

fn resolve_module_imports(
    syn_module: &syntax_ast::Module,
    sem_module: &Rc<Module>,
    sem_ast: &mut Ast,
) -> Result<(), Error> {
    for dep in syn_module.dependencies.iter() {
        if !sem_ast.modules.contains_key(dep) {
            return Err(Error {
                typ: ErrorType::Import,
                msg: "Dependency doesn't exist",
                span: Span::path_only(syn_module.path),
            });
        }
    }
    for (file_name, syn_file) in syn_module.files.iter() {
        let sem_file = sem_module.files.get_mut(file_name).unwrap();
        for (import, span) in syn_file.imports.iter() {
            if !syn_module.dependencies.contains(import) {
                return Err(Error {
                    typ: ErrorType::Import,
                    msg: "Importing undeclared module",
                    span: *span,
                });
            }
            let imported = sem_ast.modules.get(import).unwrap();
            sem_file.imports.insert(*import, imported.clone());
        }
    }
    Ok(())
}
