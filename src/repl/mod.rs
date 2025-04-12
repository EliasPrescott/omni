use std::io::{stdin, stdout, BufRead, Write};
use std::path::Path;
use std::rc::Rc;

use crate::interpreter::environment::OmniEnvironment;
use crate::interpreter::registry::FileStoreRegistry;
use crate::parsers::parse;

pub fn repl() {
    let environment = Rc::new(OmniEnvironment::new());
    let registry = FileStoreRegistry::try_create(Path::new("./.omni")).unwrap();
    loop {
        let mut stdin = stdin().lock();
        let mut stdout = stdout().lock();
        write!(stdout, "> ").unwrap();
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let expr = parse(&input).unwrap();
        let result = expr.eval(environment.clone(), &registry);
        writeln!(stdout, "{}", result.format_min()).unwrap();
    }
}
