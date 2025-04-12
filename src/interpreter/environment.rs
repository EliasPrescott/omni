use std::collections::HashMap;
use std::rc::Rc;

use crate::core_types::OmniType;
use crate::registry::OmniRegistry;

#[derive(Debug, Clone)]
pub struct OmniEnvironment {
    parent: Option<Rc<OmniEnvironment>>,
    bindings: HashMap<String, OmniType>,
    inside_quasiquote: bool,
    inside_format_quasiquote: bool,
}

impl OmniEnvironment {
    pub fn can_unquote(&self) -> bool {
        self.inside_quasiquote
    }

    pub fn can_format_unquote(&self) -> bool {
        self.inside_format_quasiquote
    }

    pub fn with_format_quasiquote(self: Rc<Self>) -> Self {
        OmniEnvironment {
            bindings: HashMap::new(),
            parent: Some(self.clone()),
            inside_quasiquote: self.inside_quasiquote,
            inside_format_quasiquote: true,
        }
    }


    pub fn with_quasiquote(self: Rc<Self>) -> Self {
        OmniEnvironment {
            bindings: HashMap::new(),
            parent: Some(self.clone()),
            inside_quasiquote: true,
            inside_format_quasiquote: self.inside_format_quasiquote,
        }
    }

    pub fn add_bindings(self: Rc<Self>, bindings: Vec<(String, OmniType)>) -> Self {
        let mut new = OmniEnvironment {
            bindings: HashMap::new(),
            parent: Some(self.clone()),
            inside_quasiquote: self.inside_quasiquote,
            inside_format_quasiquote: self.inside_format_quasiquote,
        };
        for (key, value) in bindings {
            new.bindings.insert(key, value);
        }
        new
    }

    pub fn all_bindings(&self) -> Vec<(String, OmniType)> {
        let mut bindings: Vec<(String, OmniType)> = self.bindings.clone().into_iter().collect();
        match &self.parent {
            Some(parent) => {
                let mut parent_bindings = parent.all_bindings();
                bindings.append(&mut parent_bindings);
                bindings
            }
            None => bindings
        }
    }

    pub fn store_state(self: &Rc<Self>, registry: &dyn OmniRegistry) -> String {
        let state = self.all_bindings();
        let resolved_bindings: Vec<OmniType> = state.into_iter()
            .map(|(key, value)| {
                let hash = registry.store(&value, self.clone()).unwrap();
                OmniType::List(vec![OmniType::Quote(Box::new(OmniType::Symbol(key))), OmniType::Hash(hash)])
            })
            .collect();
        let resolved_state = OmniType::List(resolved_bindings);
        let store_hash = registry.store(&resolved_state, self.clone()).unwrap();
        store_hash
    }

    pub fn get(&self, key: &str) -> Option<OmniType> {
        if key == "*state*" {
            let bindings: Vec<OmniType> = self.all_bindings().into_iter()
                .map(|(key, value)| OmniType::List(vec![OmniType::Symbol(key), value]))
                .collect();
            let expr = OmniType::List(bindings);
            return Some(expr);
        }

        match self.bindings.get(key) {
            None => {
                match &self.parent {
                    None => None,
                    Some(parent) => parent.get(key),
                }
            },
            Some(value) => Some(value.clone()),
        }
    }

    pub fn new() -> Self {
        let bindings = HashMap::new();
        OmniEnvironment {
            bindings,
            parent: None,
            inside_quasiquote: false,
            inside_format_quasiquote: false,
        }
    }
}
