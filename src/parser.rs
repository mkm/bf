use crate::syntax::{Cmd, Program};

pub fn parse_program(source: &str) -> Result<Program, ()> {
    let mut program_stack = vec![Vec::new()];
    for c in source.chars() {
        match c {
            '+' => {
                program_stack.last_mut().unwrap().push(Cmd::Inc);
            }
            '-' => {
                program_stack.last_mut().unwrap().push(Cmd::Dec);
            }
            '>' => {
                program_stack.last_mut().unwrap().push(Cmd::Fwd);
            }
            '<' => {
                program_stack.last_mut().unwrap().push(Cmd::Bwd);
            }
            ',' => {
                program_stack.last_mut().unwrap().push(Cmd::In);
            }
            '.' => {
                program_stack.last_mut().unwrap().push(Cmd::Out);
            }
            '[' => {
                program_stack.push(Vec::new());
            }
            ']' => {
                let body = program_stack.pop().unwrap();
                program_stack.last_mut().unwrap().push(Cmd::Loop(body));
            }
            _ => {}
        }
    }
    let program = program_stack.pop().unwrap();
    assert!(program_stack.is_empty());
    Ok(program)
}
