use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::syntax_ast::{Ast, Module};

use super::*;

impl SyntacticParser {
    fn path_to_module_name(&mut self, path: &Path) -> SymbolId {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        self.lexer.symbol_table.insert(name)
    }

    fn read_file(path: PathBuf) -> Result<String, Error> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(err) => Err(Error {
                typ: ErrorType::Io(Box::new(err)),
                msg: "Failed to open module file",
                token: None,
            }),
        }
    }

    fn parse_module_file(
        &mut self,
        module_path: &Path,
        queue: &mut HashSet<PathBuf>,
        modules: &HashMap<SymbolId, Module>,
    ) -> Result<HashSet<SymbolId>, Error> {
        let module_file = module_path.join("module.json");
        let content = SyntacticParser::read_file(module_file)?;
        let dependencies: Vec<String> = match serde_json::from_str(&content) {
            Ok(dependencies) => dependencies,
            Err(err) => {
                return Err(Error {
                    typ: ErrorType::ModuleFile(Box::new(err)),
                    msg: "Invalid module file",
                    token: None,
                });
            }
        };
        let mut ret = HashSet::with_capacity(dependencies.len());
        for dep in dependencies {
            let path = PathBuf::from_str(&dep).unwrap();
            let name = self.path_to_module_name(&path);
            if queue.contains(&path) || modules.contains_key(&name) {
                continue;
            }
            ret.insert(name);
            queue.insert(path);
        }
        Ok(ret)
    }

    fn read_dir(dir: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>), Error> {
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(err) => {
                return Err(Error {
                    typ: ErrorType::Io(Box::new(err)),
                    msg: "Failed to read dir",
                    token: None,
                });
            }
        };
        for entry in entries {
            if entry.is_err() {
                continue;
            }
            let path = entry.unwrap().path();
            if path.is_file() {
                if path.extension().is_none() {
                    continue;
                }
                let ext = match path.extension().unwrap().to_str() {
                    Some(ext) => ext,
                    None => continue,
                };
                if ext != "code" {
                    continue;
                }
                files.push(path);
            } else if path.is_dir() && path.join("module.json").exists() {
                dirs.push(path);
            }
        }
        Ok((files, dirs))
    }

    fn parse_module(
        &mut self,
        module_path: &Path,
        queue: &mut HashSet<PathBuf>,
        modules: &HashMap<SymbolId, Module>,
    ) -> Result<Module, Error> {
        let dependencies = self.parse_module_file(module_path, queue, modules)?;
        let mut files = HashMap::new();
        let (file_paths, module_paths) = SyntacticParser::read_dir(module_path)?;
        let module_name = self.path_to_module_name(module_path);
        for path in file_paths {
            let code = match fs::read_to_string(&path) {
                Ok(code) => code,
                Err(err) => {
                    return Err(Error {
                        typ: ErrorType::Io(Box::new(err)),
                        msg: "Failed to read file",
                        token: None,
                    });
                }
            };
            let filename = self.path_to_module_name(&path);
            let file = self.parse_code(&code, filename, module_name)?;
            files.insert(filename, file);
        }
        let mut submodules = HashMap::new();
        for path in module_paths {
            let name = self.path_to_module_name(&path);
            let submodule = self.parse_module(&path, queue, modules)?;
            submodules.insert(name, submodule);
        }
        Ok(Module {
            name: module_name,
            files,
            modules: submodules,
            dependencies,
        })
    }

    pub(crate) fn parse_modules(&mut self, module_path: &Path) -> Result<Ast, Error> {
        let entry = module_path.to_path_buf();
        let mut queue = HashSet::new();
        let mut modules = HashMap::new();
        queue.insert(entry.clone());
        while !queue.is_empty() {
            let path = queue.iter().next().unwrap().to_owned();
            if let Some(parent) = path.parent()
                && parent.join("module.json").exists()
            {
                return Err(Error {
                    typ: ErrorType::Module,
                    msg: "Importing non-top-level module",
                    token: None,
                });
            }
            let module = self.parse_module(&path, &mut queue, &modules)?;
            modules.insert(self.path_to_module_name(&path), module);
            queue.remove(&path);
        }
        Ok(Ast {
            entry: self.path_to_module_name(&entry),
            modules,
        })
    }
}
