use std::collections::HashMap;
use std::rc::Rc;

use crate::core_types::OmniType;

#[derive(Debug, Clone)]
pub struct OmniEnvironment {
    parent: Option<Rc<OmniEnvironment>>,
    bindings: HashMap<String, OmniType>,
}

impl OmniEnvironment {
    pub fn add_bindings(self: Rc<Self>, bindings: Vec<(String, OmniType)>) -> Self {
        let mut new = OmniEnvironment {
            bindings: HashMap::new(),
            parent: None,
        };
        for (key, value) in bindings {
            new.bindings.insert(key, value);
        }
        new.parent = Some(self);
        new
    }

    pub fn get(&self, key: &str) -> Option<&OmniType> {
        match self.bindings.get(key) {
            None => {
                match &self.parent {
                    None => None,
                    Some(parent) => parent.get(key),
                }
            },
            Some(value) => Some(value),
        }
    }

    pub fn new() -> Self {
        let mut bindings = HashMap::new();
        OmniEnvironment {
            bindings,
            parent: None,
        }
    }
}
