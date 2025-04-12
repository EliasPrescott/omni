use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::core_types::OmniType;
use crate::parsers::parse;

pub trait OmniRegistry {
    fn resolve(&self, hash: &String) -> Option<OmniType>;
    fn store(&self, expr: &OmniType) -> Result<String, String>;
}

pub struct NullRegistry;

impl OmniRegistry for NullRegistry {
    fn resolve(&self, _hash: &String) -> Option<OmniType> {
        None
    }

    fn store(&self, _expr: &OmniType) -> Result<String, String> {
        Err(String::from("Cannot store expressions in the null registry"))
    }
}

pub struct FileStoreRegistry {
    directory_path: PathBuf,
}

impl FileStoreRegistry {
    pub fn try_create(path: &Path) -> Result<Self, String> {
        let metadata = std::fs::metadata(path).unwrap();
        if !metadata.is_dir() {
            return Err(String::from("Path must point to a directory"));
        }
        Ok(FileStoreRegistry {
            directory_path: path.to_path_buf(),
        })
    }
}

impl OmniRegistry for FileStoreRegistry {
    fn resolve(&self, hash: &String) -> Option<OmniType> {
        let mut file_path = self.directory_path.clone();
        file_path.push(hash);
        match std::fs::File::open(file_path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let expr = parse(&contents).unwrap();
                // Recalculating the hash is expensive to do at runtime, but it's a cool
                // way of verifying the registry is not lying.
                let registry_hash = expr.hash();
                assert_eq!(&registry_hash, hash);
                Some(expr)
            },
            Err(_) => {
                None
            }
        }
    }

    fn store(&self, expr: &OmniType) -> Result<String, String> {
        let code = expr.format_min();
        let hash = expr.hash();
        let mut file_path = self.directory_path.clone();
        file_path.push(&hash);
        std::fs::write(file_path, code).unwrap();
        Ok(hash)
    }
}
