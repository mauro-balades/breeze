use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonCache {
    pub has_installed_dependencies: bool,
    pub last_installed_dependencies: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cache {
    pub python: PythonCache,
}

impl Cache {
    pub fn new() -> Cache {
        Self {
            python: PythonCache {
                has_installed_dependencies: false,
                last_installed_dependencies: 0,
            },
        }
    }
}
