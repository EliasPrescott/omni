use std::collections::HashMap;
use std::rc::Rc;

use crate::core_types::OmniType;

#[derive(Debug, Clone)]
pub struct OmniEnvironment {
    parent: Option<Rc<OmniEnvironment>>,
    bindings: HashMap<String, OmniType>,
    inside_quasiquote: bool,
}

impl OmniEnvironment {
    pub fn can_unquote(&self) -> bool {
        self.inside_quasiquote
    }

    pub fn with_quasiquote(self: Rc<Self>) -> Self {
        OmniEnvironment {
            bindings: HashMap::new(),
            parent: Some(self.clone()),
            inside_quasiquote: true,
        }
    }

    pub fn add_bindings(self: Rc<Self>, bindings: Vec<(String, OmniType)>) -> Self {
        let mut new = OmniEnvironment {
            bindings: HashMap::new(),
            parent: Some(self.clone()),
            inside_quasiquote: self.inside_quasiquote,
        };
        for (key, value) in bindings {
            new.bindings.insert(key, value);
        }
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
        let bindings = HashMap::new();
        OmniEnvironment {
            bindings,
            parent: None,
            inside_quasiquote: false,
        }
    }
}
