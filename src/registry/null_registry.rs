use std::rc::Rc;

use crate::core_types::OmniType;
use crate::interpreter::environment::OmniEnvironment;

use super::OmniRegistry;

pub struct NullRegistry;

impl OmniRegistry for NullRegistry {
    fn resolve(&self, _hash: &String) -> Option<OmniType> {
        None
    }

    fn store(&self, _expr: &OmniType, _: Rc<OmniEnvironment>) -> Result<String, String> {
        Err(String::from("Cannot store expressions in the null registry"))
    }
}
