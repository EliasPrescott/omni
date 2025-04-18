use std::rc::Rc;

use crate::core_types::OmniType;
use crate::registry::OmniRegistry;

use self::environment::OmniEnvironment;

pub mod environment;
mod tests;

pub struct LambdaDetails {
    pub args: Vec<String>,
    pub body: OmniType,
}

#[derive(Debug)]
pub struct MacroDetails {
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

    fn unwrap_as_hash(self: OmniType) -> String {
        match self {
            OmniType::Hash(hash) => hash,
            other => panic!("{:?} is not a hash", other),
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

    fn unwrap_as_macro(self: OmniType) -> MacroDetails {
        match self {
            OmniType::List(items) => {
                let hopefully_macro_symbol = items.first().unwrap().clone().unwrap_as_symbol();
                assert_eq!(hopefully_macro_symbol, "macro");
                let args: Vec<String> = items[1].clone().unwrap_as_list().into_iter().map(|x| x.unwrap_as_symbol()).collect();
                MacroDetails {
                    args,
                    body: items[2].clone(),
                }
            },
            other => panic!("{:?} is not a macro expr", other),
        }
    }


    pub fn unquote(self: &OmniType, environment: Rc<OmniEnvironment>, registry: &dyn OmniRegistry) -> Vec<OmniType> {
        match self {
            OmniType::UnQuote(item) => {
                vec![item.eval(environment, registry).0]
            }
            OmniType::Spread(item) => {
                item.eval(environment, registry).0.unwrap_as_list()
            }
            other => vec![other.clone()]
        }
    }

    pub fn eval(self: &OmniType, environment: Rc<OmniEnvironment>, registry: &dyn OmniRegistry) -> (OmniType, Rc<OmniEnvironment>) {
        match self {
            OmniType::Quote(inner) => (*inner.clone(), environment),
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
                (OmniType::List(items), environment)
            }
            OmniType::Int(num) => (OmniType::Int(*num), environment),
            OmniType::Hash(hash) => (registry.resolve(hash).expect(&format!("Could not resolve ${}", hash)), environment),
            OmniType::List(items) => {
                let first = items.first().unwrap();
                match first {
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "store" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry);
                        let hash = registry.store(&x.0, environment.clone()).unwrap();
                        (OmniType::Hash(hash), environment)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "store-state" => {
                        let hash = environment.store_state(registry);
                        (OmniType::Hash(hash), environment)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "load-state" => {
                        let _given_hash = items.get(1).unwrap().eval(environment.clone(), registry).0.unwrap_as_hash();
                        todo!();
                        // (OmniType::Hash(hash), environment)
                    }

                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "def" => {
                        let name = items.get(1).unwrap().clone().unwrap_as_symbol();
                        let expr = items.get(2).unwrap().eval(environment.clone(), registry);
                        let bindings = vec![(name, expr.0.clone())];
                        let environment = environment.add_bindings(bindings);
                        (expr.0, Rc::new(environment))
                    }

                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "first" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).0.unwrap_as_list();
                        let first = x.get(0).unwrap().clone();
                        (first, environment)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "last" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).0.unwrap_as_list();
                        let last = x.last().unwrap().clone();
                        (last, environment)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "nth" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).0.unwrap_as_list();
                        let y = items.get(2).unwrap().eval(environment.clone(), registry).0.unwrap_as_int();
                        let nth = x.get(y as usize).unwrap().clone();
                        (nth, environment)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "rest" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).0.unwrap_as_list();
                        let rest = x.into_iter().skip(1).collect::<Vec<_>>();
                        (OmniType::List(rest), environment)
                    }

                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "+" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).0.unwrap_as_int();
                        let y = items.get(2).unwrap().eval(environment.clone(), registry).0.unwrap_as_int();
                        (OmniType::Int(x + y), environment)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "-" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).0.unwrap_as_int();
                        let y = items.get(2).unwrap().eval(environment.clone(), registry).0.unwrap_as_int();
                        (OmniType::Int(x - y), environment)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "*" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).0.unwrap_as_int();
                        let y = items.get(2).unwrap().eval(environment.clone(), registry).0.unwrap_as_int();
                        (OmniType::Int(x * y), environment)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "/" => {
                        let x = items.get(1).unwrap().eval(environment.clone(), registry).0.unwrap_as_int();
                        let y = items.get(2).unwrap().eval(environment.clone(), registry).0.unwrap_as_int();
                        (OmniType::Int(x / y), environment)
                    }
                    OmniType::Symbol(builtin_symbol) if builtin_symbol == "defmacro" => {
                        let macro_name = items.get(1).unwrap().clone().unwrap_as_symbol();
                        let args: Vec<String> = items.get(2).unwrap().clone()
                            .unwrap_as_list()
                            .into_iter()
                            .map(|x| x.unwrap_as_symbol())
                            .collect();
                        let body = items.get(3).unwrap();
                        let expr_list = vec![
                            OmniType::Symbol(String::from("macro")),
                            OmniType::List(args.into_iter().map(|x| OmniType::Symbol(x)).collect()),
                            body.clone(),
                        ];
                        let macro_expr = OmniType::List(expr_list);
                        let environment = Rc::new(environment.add_bindings(vec![(macro_name, macro_expr)]));
                        (OmniType::List(vec![]), environment)
                    }
                    OmniType::Hash(hash) => {
                        let first = registry.resolve(hash).unwrap();
                        let mut expr_list = vec![first];
                        for item in items.into_iter().skip(1) {
                            expr_list.push(item.clone());
                        }
                        OmniType::List(expr_list).eval(environment, registry)
                    }
                    OmniType::Symbol(_) => {
                        let first = first.eval(environment.clone(), registry).0;
                        let mut expr_list = vec![first];
                        for item in items.into_iter().skip(1) {
                            expr_list.push(item.clone());
                        }
                        OmniType::List(expr_list).eval(environment, registry)
                    },
                    OmniType::List(_) => {
                        let first_arg_list = first.clone().unwrap_as_list();
                        let first_arg_list_first_symbol = first_arg_list.get(0).cloned().unwrap()
                            .unwrap_as_symbol();
                        if first_arg_list_first_symbol == "lambda" {
                            let lambda_details = first.clone().unwrap_as_lambda();
                            let arg_exprs = &items[1..];
                            assert_eq!(arg_exprs.len(), lambda_details.args.len());
                            let arg_exprs: Vec<OmniType> = arg_exprs.into_iter().map(|x| x.eval(environment.clone(), registry).0).collect();
                            let new_bindings: Vec<(String, OmniType)> = lambda_details.args.clone().into_iter().zip(arg_exprs).collect();
                            let new_env = Rc::new(OmniEnvironment::add_bindings(environment, new_bindings));
                            lambda_details.body.eval(new_env, registry)
                        } else {
                            let macro_details = first.clone().unwrap_as_macro();
                            let arg_exprs = &items[1..];
                            assert_eq!(arg_exprs.len(), macro_details.args.len());
                            let arg_exprs: Vec<OmniType> = arg_exprs.to_vec();
                            let new_bindings: Vec<(String, OmniType)> = macro_details.args.clone().into_iter().zip(arg_exprs).collect();
                            let new_env = Rc::new(OmniEnvironment::add_bindings(environment, new_bindings));
                            macro_details.body.eval(new_env, registry)
                        }
                    },
                    other => panic!("Cannot evaluate {:?} as a function", other)
                }
            },
            OmniType::Symbol(symbol) => {
                match environment.get(symbol) {
                    None => panic!("Could not evaluate symbol {}", symbol),
                    Some(expr) => (expr.clone(), environment),
                }
            },
        }
    }
}
