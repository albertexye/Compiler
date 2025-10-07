use super::*;
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Deserialize)]
struct Schema {
    pub(crate) dependencies: HashMap<String, String>,
}

pub(crate) struct Definition {
    pub(crate) dependencies: HashMap<String, PathBuf>,
}

impl Definition {
    pub(crate) fn read_definition(directory: &Path) -> Result<Definition, Error> {
        let path = directory.join("mod.json");
        if !path.exists() {
            return Err(Error::ModuleNotFound);
        }
        let Ok(json) = fs::read_to_string(path) else {
            return Err(Error::FailedToReadModule);
        };
        let schema: Result<Schema, serde_json::Error> = serde_json::from_str(&json);
        let schema = match schema {
            Ok(schema) => schema,
            Err(_) => return Err(Error::InvalidSchema),
        };
        let mut dependencies = HashMap::new();
        for (name, path) in schema.dependencies {
            let path = Path::new(&path).to_path_buf();
            dependencies.insert(name, path);
        }
        Ok(Definition { dependencies })
    }
}
