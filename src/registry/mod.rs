use std::rc::Rc;

use crate::core_types::OmniType;
use crate::interpreter::environment::OmniEnvironment;

pub mod null_registry;
pub mod file_store_registry;

pub trait OmniRegistry {
    fn resolve(&self, hash: &String) -> Option<OmniType>;
    fn store(&self, expr: &OmniType, environment: Rc<OmniEnvironment>) -> Result<String, String>;
}

