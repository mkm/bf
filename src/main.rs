use bf::bir::Stmt;
use bf::interpreter::Machine;
use bf::optimiser;
use bf::parser;
use bf::printer;
use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).unwrap();
    let source = fs::read_to_string(path).unwrap();
    let program = parser::parse_program(&source).unwrap();
    let bir = Stmt::from_program(program.clone());
    let mut machine = Machine::new();
    let bir = optimiser::opt_bir(bir);
    printer::print_bir(&bir);
    println!();
    machine.run_bir(&bir);
}
