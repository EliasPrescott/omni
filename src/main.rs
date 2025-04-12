mod core_types;
mod parsers;
mod formatter;
mod registry;
mod interpreter;
mod repl;

fn main() {
    repl::repl();
}
