#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::core_types::OmniType;
    use crate::interpreter::environment::OmniEnvironment;
    use crate::interpreter::registry::NullRegistry;
    use crate::parsers::parse;

    #[test]
    fn eval_lambda() {
        let identity_lambda = OmniType::List(vec![
            OmniType::Symbol(String::from("lambda")),
            OmniType::List(vec![
                OmniType::Symbol(String::from("x")),
            ]),
            OmniType::Symbol(String::from("x"))
        ]);
        let funcall = OmniType::List(vec![
            identity_lambda,
            OmniType::Int(42),
        ]);
        let environment = Rc::new(OmniEnvironment::new());
        let registry = NullRegistry;
        let result = funcall.eval(environment, &registry);
        assert_eq!(result, OmniType::Int(42));
    }

    #[test]
    fn parse_and_eval_identity() {
        let environment = Rc::new(OmniEnvironment::new());
        let registry = NullRegistry;
        let expr = parse("((lambda (x) x) 123)").unwrap();
        let result = expr.eval(environment, &registry);
        assert_eq!(result, OmniType::Int(123));
    }
}
