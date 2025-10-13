use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::lexer::Lexer;
use crate::syntactic_parser::SyntacticParser;
use crate::syntax_ast::{Ast, File, Module};
use crate::{lexer, syntactic_parser};

#[derive(Deserialize)]
struct Schema {
    pub(crate) dependencies: HashMap<String, String>,
}

struct Definition {
    pub(crate) dependencies: HashMap<String, PathBuf>,
}

enum ErrorType {
    Io(io::Error),
    Lex(lexer::Error),
    Syntax(syntactic_parser::Error),
    ModuleSchema(serde_json::Error),
    ModuleNotFound,
}

struct Error {
    typ: ErrorType,
    msg: String,
}

fn path_to_string(path: &Path) -> Result<String, Error> {
    match path.to_str() {
        Some(path_str) => Ok(path_str.to_string()),
        None => Err(Error {
            typ: ErrorType::Io(io::Error::new(
                io::ErrorKind::InvalidFilename,
                "Paths must be UTF-8 compatible",
            )),
            msg: "Invalid path".to_string(),
        }),
    }
}

fn read_file(path: &Path) -> Result<String, Error> {
    let path_str = path_to_string(path)?;
    if !path.exists() {
        return Err(Error {
            typ: ErrorType::ModuleNotFound,
            msg: format!("Module {} not found", path_str),
        });
    }
    match fs::read_to_string(path) {
        Ok(json) => Ok(json),
        Err(err) => Err(Error {
            typ: ErrorType::Io(err),
            msg: format!("Failed to read module file: {}", path_str),
        }),
    }
}

fn parse_definition(directory: &Path) -> Result<Definition, Error> {
    let path = directory.join("mod.json");
    let json = read_file(&path)?;
    let schema: Result<Schema, serde_json::Error> = serde_json::from_str(&json);
    let schema = match schema {
        Ok(schema) => schema,
        Err(err) => {
            return Err(Error {
                typ: ErrorType::ModuleSchema(err),
                msg: "Invalid schema".to_string(),
            });
        }
    };
    let mut dependencies = HashMap::new();
    for (name, path) in schema.dependencies {
        let mut path = PathBuf::from(&path);
        if path.is_relative() {
            let mut base = directory.to_path_buf();
            base.push(path);
            path = base;
        }
        dependencies.insert(name, path);
    }
    Ok(Definition { dependencies })
}

fn wrap_io_result<T>(result: Result<T, io::Error>, msg: String) -> Result<T, Error> {
    match result {
        Ok(v) => Ok(v),
        Err(e) => Err(Error {
            typ: ErrorType::Io(e),
            msg,
        }),
    }
}

fn get_source_files(path: &Path) -> Result<Vec<PathBuf>, Error> {
    let path_str = path_to_string(path)?;
    let entries = wrap_io_result(
        fs::read_dir(path),
        format!("Failed to read path: {}", path_str),
    )?;
    let mut ret = Vec::new();
    for entry in entries {
        let entry = wrap_io_result(entry, format!("Failed to read entry in path: {}", path_str))?;
        let file_type = wrap_io_result(
            entry.file_type(),
            format!("Failed to read entry in path: {}", path_str),
        )?;
        if !file_type.is_file() {
            continue;
        }
        let file_path = entry.path();
        let file_path_str = path_to_string(&file_path)?;
        if !file_path_str.ends_with(".sc") {
            continue;
        }
        ret.push(file_path);
    }
    Ok(ret)
}

fn compile_file(path: &Path, module_name: &str) -> Result<(String, File), Error> {
    let path_str = path.file_name().unwrap().to_str().unwrap();
    let filename = path_str[..path_str.len() - 3].to_string();
    let code = wrap_io_result(
        fs::read_to_string(path),
        format!("Failed to read file: {}", path_str),
    )?;
    let tokens = match Lexer::lex(&code) {
        Ok(tokens) => tokens,
        Err(err) => {
            return Err(Error {
                typ: ErrorType::Lex(err),
                msg: "Lexical analysis error".to_string(),
            });
        }
    };
    match SyntacticParser::parse(tokens, &filename, module_name) {
        Ok(file_ast) => Ok((filename, file_ast)),
        Err(err) => Err(Error {
            typ: ErrorType::Syntax(err),
            msg: "Syntactic analysis error".to_string(),
        }),
    }
}

fn compile_module(path: &Path) -> Result<Module, Error> {
    let definition = parse_definition(path)?;
    let files = get_source_files(path)?;
    let module_name = path.file_name().unwrap().to_str().unwrap().to_string();
    let mut file_map = HashMap::new();
    for file in files {
        let (filename, file_ast) = compile_file(&file, &module_name)?;
        file_map.insert(filename, file_ast);
    }
    Ok(Module {
        name: module_name,
        files: file_map,
        dependencies: definition.dependencies,
    })
}

pub(crate) fn compile(path: &Path) -> Result<Ast, Error> {
    let mut modules = HashMap::new();
    let mut queue = HashSet::new();
    let path = if path.is_relative() {
        match std::path::absolute(path) {
            Ok(path) => path,
            Err(err) => {
                return Err(Error {
                    typ: ErrorType::Io(err),
                    msg: format!("Invalid path: {}", path_to_string(path)?),
                });
            }
        }
    } else {
        path.to_path_buf()
    };
    queue.insert(path.clone());
    while !queue.is_empty() {
        let path = queue.iter().next().unwrap().clone();
        let module = compile_module(&path)?;
        for dep in module.dependencies.values() {
            if !modules.contains_key(dep) && !queue.contains(dep) {
                queue.insert(dep.clone());
            }
        }
        queue.remove(&path);
        modules.insert(path, module);
    }
    Ok(Ast {
        entry: path,
        modules,
    })
}
