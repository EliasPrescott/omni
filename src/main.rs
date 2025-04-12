mod core_types;
mod parsers;
mod formatter;
mod interpreter;
mod repl;

fn main() {
    repl::repl();
}
