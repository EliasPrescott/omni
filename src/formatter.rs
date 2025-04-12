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
            OmniType::QuasiQuote(items) => {
                let formatted_items: Vec<String> = items.into_iter().map(|x| x.format_min()).collect();
                format!("`{}", formatted_items.join(" "))
            },
            OmniType::UnQuote(item) => format!(",{}", item.format_min()),
            OmniType::Spread(item) => format!(",@{}", item.format_min()),
        }
    }
}
