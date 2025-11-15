use super::*;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};
use syntax_ast::{Ast, Module};

impl SyntacticParser {
    fn path_to_module_name(path: &Path, pool: &mut InternPool) -> SymbolId {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        pool.insert_symbol(name)
    }

    fn path_to_filename(path: &Path, pool: &mut InternPool) -> SymbolId {
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        pool.insert_symbol(name)
    }

    fn read_file(path: &Path, pool: &mut InternPool) -> Result<String, Error> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(err) => Err(Error {
                typ: ErrorType::Io(Box::new(err)),
                msg: "Failed to open module file",
                span: Span::path_only(pool.insert_path(path.to_path_buf())),
            }),
        }
    }

    fn parse_module_file(
        module_path: &Path,
        queue: &mut HashSet<PathBuf>,
        modules: &HashMap<SymbolId, Module>,
        pool: &mut InternPool,
    ) -> Result<HashSet<SymbolId>, Error> {
        let module_file = module_path.join("module.json");
        let content = Self::read_file(&module_file, pool)?;
        let dependencies: Vec<String> = match serde_json::from_str(&content) {
            Ok(dependencies) => dependencies,
            Err(err) => {
                return Err(Error {
                    typ: ErrorType::ModuleFile(Box::new(err)),
                    msg: "Invalid module file",
                    span: Span::path_only(pool.insert_path(module_file)),
                });
            }
        };
        let mut ret = HashSet::with_capacity(dependencies.len());
        for dep in dependencies {
            let path = PathBuf::from_str(&dep).unwrap();
            let name = SyntacticParser::path_to_module_name(&path, pool);
            if queue.contains(&path) || modules.contains_key(&name) {
                continue;
            }
            ret.insert(name);
            queue.insert(path);
        }
        Ok(ret)
    }

    fn read_dir(dir: &Path, pool: &mut InternPool) -> Result<(Vec<PathBuf>, Vec<PathBuf>), Error> {
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(err) => {
                return Err(Error {
                    typ: ErrorType::Io(Box::new(err)),
                    msg: "Failed to read dir",
                    span: Span::path_only(pool.insert_path(dir.to_path_buf())),
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
        module_path: &Path,
        queue: &mut HashSet<PathBuf>,
        modules: &HashMap<SymbolId, Module>,
        pool: &mut InternPool,
    ) -> Result<Module, Error> {
        let dependencies = Self::parse_module_file(module_path, queue, modules, pool)?;
        let mut files = HashMap::new();
        let (file_paths, module_paths) = Self::read_dir(module_path, pool)?;
        let module_name = Self::path_to_module_name(module_path, pool);
        for path in file_paths {
            let code = match fs::read_to_string(&path) {
                Ok(code) => code,
                Err(err) => {
                    return Err(Error {
                        typ: ErrorType::Io(Box::new(err)),
                        msg: "Failed to read file",
                        span: Span::path_only(pool.insert_path(path)),
                    });
                }
            };
            let filename = Self::path_to_module_name(&path, pool);
            let path_id = pool.insert_path(path);
            let file = Self::parse_code(path_id, &code, filename, module_name, pool)?;
            files.insert(filename, file);
        }
        let mut submodules = HashMap::new();
        for path in module_paths {
            let name = Self::path_to_module_name(&path, pool);
            if files.contains_key(&name) {
                return Err(Error {
                    typ: ErrorType::Module,
                    msg: "Submodule has the same name as a file",
                    span: Span::path_only(pool.insert_path(path)),
                });
            }
            let submodule = Self::parse_module(&path, queue, modules, pool)?;
            submodules.insert(name, submodule);
        }
        Ok(Module {
            path: pool.insert_path(module_path.to_path_buf()),
            name: module_name,
            files,
            submodules,
            dependencies,
        })
    }

    pub(crate) fn parse_modules(module_path: &Path, pool: &mut InternPool) -> Result<Ast, Error> {
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
                    span: Span::path_only(pool.insert_path(path)),
                });
            }
            let module = Self::parse_module(&path, &mut queue, &modules, pool)?;
            modules.insert(Self::path_to_module_name(&path, pool), module);
            queue.remove(&path);
        }
        Ok(Ast {
            entry: Self::path_to_module_name(&entry, pool),
            modules,
        })
    }
}
