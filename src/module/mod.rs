use std::collections::HashMap;
use std::path::{Path, PathBuf};

use definition::Definition;

mod definition;

pub(crate) enum Error {
    ModuleNotFound,
    FailedToReadModule,
    InvalidSchema,
}

pub(crate) fn resolve(path: &Path) -> Result<HashMap<PathBuf, Definition>, Error> {
    let mut resolved = HashMap::new();
    let mut queue = Vec::new();
    queue.push(path.to_path_buf());
    while let Some(path) = queue.pop() {
        let def = Definition::read_definition(&path)?;
        for dep in def.dependencies.values() {
            queue.push(dep.to_path_buf());
        }
        resolved.insert(path, def);
    }
    Ok(resolved)
}
