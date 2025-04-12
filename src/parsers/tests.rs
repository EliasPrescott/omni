#[cfg(test)]
mod tests {
    use crate::core_types::OmniType;
    use crate::parsers::parse_omni_expr;

    #[test]
    fn parse_hash() {
        let hash = String::from("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
        let (_, result) = parse_omni_expr(format!("${}", hash).as_str()).unwrap();
        assert_eq!(result, OmniType::Hash(hash));
    }

    #[test]
    fn parse_int() {
        let (_, result) = parse_omni_expr("42").unwrap();
        assert_eq!(result, OmniType::Int(42));
    }

    #[test]
    fn parse_list() {
        let (_, result) = parse_omni_expr("(1 2 3)").unwrap();
        assert_eq!(result, OmniType::List(vec![
            OmniType::Int(1),
            OmniType::Int(2),
            OmniType::Int(3),
        ]));
    }

    #[test]
    fn parse_symbol() {
        let (_, result) = parse_omni_expr("(+ plus print)").unwrap();
        assert_eq!(result, OmniType::List(vec![
            OmniType::Symbol(String::from("+")),
            OmniType::Symbol(String::from("plus")),
            OmniType::Symbol(String::from("print")),
        ]));
    }

    #[test]
    fn parse_quote() {
        let (_, result) = parse_omni_expr("'lambda").unwrap();
        assert_eq!(result, OmniType::Quote(Box::new(OmniType::Symbol(String::from("lambda")))));
    }
}
