use std::io::{stdin, stdout, BufRead, Write};
use std::path::Path;
use std::rc::Rc;

use crate::interpreter::environment::OmniEnvironment;
use crate::parsers::parse;
use crate::registry::file_store_registry::FileStoreRegistry;

pub fn repl() {
    let mut environment = Rc::new(OmniEnvironment::new());
    let registry = FileStoreRegistry::try_create(Path::new("./.omni")).unwrap();
    loop {
        let mut stdin = stdin().lock();
        let mut stdout = stdout().lock();
        write!(stdout, "> ").unwrap();
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let expr = parse(&input).unwrap();
        let (result, new_environment) = expr.eval(environment.clone(), &registry);
        environment = new_environment;
        writeln!(stdout, "{}", result.format_min()).unwrap();
    }
}
