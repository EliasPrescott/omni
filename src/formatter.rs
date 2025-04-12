use std::rc::Rc;

use crate::core_types::OmniType;
use crate::interpreter::environment::OmniEnvironment;
use crate::interpreter::registry::OmniRegistry;

impl OmniType {
    pub fn format_min(self: &Self) -> String {
        match self {
            OmniType::Int(num) => num.to_string(),
            OmniType::Hash(hash) => format!("${hash}"),
            OmniType::List(items) => {
                let formatted_items: Vec<String> = items.into_iter().map(|x| x.format_min()).collect();
                format!("({})", formatted_items.join(" "))
            },
            OmniType::Symbol(symbol) => symbol.to_owned(),
            OmniType::Quote(inner) => format!("'{}", inner.format_min()),
            OmniType::QuasiQuote(items) => {
                let formatted_items: Vec<String> = items.into_iter().map(|x| x.format_min()).collect();
                format!("`{}", formatted_items.join(" "))
            },
            OmniType::UnQuote(item) => format!(",{}", item.format_min()),
            OmniType::Spread(item) => format!(",@{}", item.format_min()),
        }
    }

    pub fn resolving_format_min(self: &Self, environment: Rc<OmniEnvironment>, registry: &dyn OmniRegistry) -> String {
        match self {
            OmniType::Int(num) => num.to_string(),
            OmniType::Hash(hash) => format!("${hash}"),
            OmniType::List(items) => {
                let formatted_items: Vec<String> = items.into_iter()
                    .map(|x| x.resolving_format_min(environment.clone(), registry)).collect();
                format!("({})", formatted_items.join(" "))
            },
            OmniType::Symbol(symbol) => {
                match environment.get(symbol) {
                    Some(expr) => {
                        let hash = registry.store(expr, environment.clone()).unwrap();
                        OmniType::Hash(hash).resolving_format_min(environment.clone(), registry)
                    }
                    // If we can't resolve the symbol, just leave it alone for now.
                    // It could be built-in, or introduced by a lambda/macro, or it could be a
                    // mistake.
                    None => {
                        symbol.to_owned()
                    }
                }
            },
            OmniType::Quote(inner) => format!("'{}", inner.resolving_format_min(environment.clone(), registry)),
            OmniType::QuasiQuote(items) => {
                let formatted_items: Vec<String> = items.into_iter()
                    .map(|x| x.resolving_format_min(environment.clone(), registry)).collect();
                format!("`{}", formatted_items.join(" "))
            },
            OmniType::UnQuote(item) => format!(",{}", item.resolving_format_min(environment.clone(), registry)),
            OmniType::Spread(item) => format!(",@{}", item.resolving_format_min(environment.clone(), registry)),
        }
    }
}
