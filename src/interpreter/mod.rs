use std::rc::Rc;

use crate::core_types::OmniType;

use self::environment::OmniEnvironment;
use self::registry::OmniRegistry;

pub mod environment;
pub mod registry;
mod tests;

pub struct LambdaDetails {
    pub args: Vec<String>,
    pub body: OmniType,
}

impl OmniType {
    fn unwrap_as_symbol(self: OmniType) -> String {
        match self {
            OmniType::Symbol(symbol) => symbol,
            other => panic!("{:?} is not a symbol", other),
        }
    }

    fn unwrap_as_list(self: OmniType) -> Vec<OmniType> {
        match self {
            OmniType::List(items) => items,
            other => panic!("{:?} is not a list", other),
        }
    }

    fn unwrap_as_int(self: &OmniType) -> i32 {
        match self {
            OmniType::Int(num) => *num,
            other => panic!("{:?} is not an int", other),
        }
    }

    fn unwrap_as_lambda(self: OmniType) -> LambdaDetails {
        match self {
            OmniType::List(items) => {
                let hopefully_lambda_symbol = items.first().unwrap().clone().unwrap_as_symbol();
                assert_eq!(hopefully_lambda_symbol, "lambda");
                let args: Vec<String> = items[1].clone().unwrap_as_list().into_iter().map(|x| x.unwrap_as_symbol()).collect();
                LambdaDetails {
                    args,
                    body: items[2].clone(),
                }
            },
            other => panic!("{:?} is not a lambda expr", other),
        }
    }

    pub fn unquote(self: &OmniType, environment: Rc<OmniEnvironment>, registry: &dyn OmniRegistry) -> Vec<OmniType> {
        match self {
            OmniType::UnQuote(item) => {
                vec![item.eval(environment, registry)]
            }
            OmniType::Spread(item) => {
                item.eval(environment, registry).unwrap_as_list()
            }
            other => vec![other.clone()]
        }
    }

    pub fn eval(self: &OmniType, environment: Rc<OmniEnvironment>, registry: &dyn OmniRegistry) -> OmniType {
        match self {
            OmniType::Quote(inner) => *inner.clone(),
            OmniType::UnQuote(item) => {
                assert!(environment.can_unquote());
                item.eval(environment, registry)
            }
            OmniType::Spread(item) => {
                assert!(environment.can_unquote());
                item.eval(environment, registry)
            }
            OmniType::QuasiQuote(items) => {
                let environment = Rc::new(environment.with_quasiquote());
                let items: Vec<OmniType> = items.into_iter().flat_map(|x| x.unquote(environment.clone(), registry)).collect();
                OmniType::List(items)
            }
            OmniType::Int(num) => OmniType::Int(*num),
            OmniType::Hash(hash) => registry.resolve(hash).expect(&format!("Could not resolve ${}", hash)),
            OmniType::List(items) => {
                let first = items.first().unwrap();
                match first {
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "store" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry);
                        let hash = registry.store(&x).unwrap();
                        OmniType::Hash(hash)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "+" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).unwrap_as_int();
                        let y = items.get(2).unwrap().eval(environment.clone(), registry).unwrap_as_int();
                        OmniType::Int(x + y)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "-" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).unwrap_as_int();
                        let y = items.get(2).unwrap().eval(environment.clone(), registry).unwrap_as_int();
                        OmniType::Int(x - y)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "*" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).unwrap_as_int();
                        let y = items.get(2).unwrap().eval(environment.clone(), registry).unwrap_as_int();
                        OmniType::Int(x * y)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "/" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).unwrap_as_int();
                        let y = items.get(2).unwrap().eval(environment.clone(), registry).unwrap_as_int();
                        OmniType::Int(x / y)
                    }
                    OmniType::Hash(hash) => {
                        let lambda_details = registry.resolve(hash).unwrap().unwrap_as_lambda();
                        let arg_exprs = &items[1..];
                        assert_eq!(arg_exprs.len(), lambda_details.args.len());
                        let arg_exprs: Vec<OmniType> = arg_exprs.into_iter().map(|x| x.eval(environment.clone(), registry)).collect();
                        let new_bindings: Vec<(String, OmniType)> = lambda_details.args.clone().into_iter().zip(arg_exprs).collect();
                        let new_env = Rc::new(OmniEnvironment::add_bindings(environment, new_bindings));
                        lambda_details.body.eval(new_env, registry)
                    }
                    OmniType::Symbol(_) => {
                        let first = first.eval(environment.clone(), registry);
                        let lambda_details = first.unwrap_as_lambda();
                        let arg_exprs = &items[1..];
                        assert_eq!(arg_exprs.len(), lambda_details.args.len());
                        let arg_exprs: Vec<OmniType> = arg_exprs.into_iter().map(|x| x.eval(environment.clone(), registry)).collect();
                        let new_bindings: Vec<(String, OmniType)> = lambda_details.args.clone().into_iter().zip(arg_exprs).collect();
                        let new_env = Rc::new(OmniEnvironment::add_bindings(environment, new_bindings));
                        lambda_details.body.eval(new_env, registry)

                    },
                    OmniType::List(_) => {
                        let lambda_details = first.clone().unwrap_as_lambda();
                        let arg_exprs = &items[1..];
                        assert_eq!(arg_exprs.len(), lambda_details.args.len());
                        let arg_exprs: Vec<OmniType> = arg_exprs.into_iter().map(|x| x.eval(environment.clone(), registry)).collect();
                        let new_bindings: Vec<(String, OmniType)> = lambda_details.args.clone().into_iter().zip(arg_exprs).collect();
                        let new_env = Rc::new(OmniEnvironment::add_bindings(environment, new_bindings));
                        lambda_details.body.eval(new_env, registry)
                    },
                    other => panic!("Cannot evaluate {:?} as a function", other)
                }
            },
            OmniType::Symbol(symbol) => {
                match environment.get(symbol) {
                    None => panic!("Could not evaluate symbol {}", symbol),
                    Some(expr) => expr.clone(),
                }
            },
        }
    }
}
