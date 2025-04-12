use crate::core_types::OmniType;

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
            OmniType::QuasiQuote(_items) => panic!("Trying to format a quasi-quote. This may be valid, but for now I want to panic and see where it's happening from."),
            OmniType::UnQuote(_item) => panic!("Trying to format an un-quote. This may be valid, but for now I want to panic and see where it's happening from."),
            OmniType::Spread(_item) => panic!("Trying to format a spread. This may be valid, but for now I want to panic and see where it's happening from."),
        }
    }
}
