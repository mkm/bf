use crate::bir::{Expr, Stmt};
use crate::syntax::{Cmd, Program};
use std::io::{self, Read, Write};

pub struct Machine {
    memory: Vec<u8>,
    head: isize,
}

impl Machine {
    pub fn new() -> Self {
        let mut memory = Vec::new();
        memory.resize(0x10000, 0);
        Self { memory, head: 0 }
    }

    pub fn run_cmd(&mut self, cmd: &Cmd) {
        match cmd {
            Cmd::Inc => {
                self.memory[self.head as usize] += 1;
            }
            Cmd::Dec => {
                self.memory[self.head as usize] -= 1;
            }
            Cmd::Fwd => {
                self.head += 1;
            }
            Cmd::Bwd => {
                self.head -= 1;
            }
            Cmd::In => {
                self.input(0);
            }
            Cmd::Out => {
                self.output(self.memory[self.head as usize]);
            }
            Cmd::Loop(program) => {
                while self.memory[self.head as usize] != 0 {
                    self.run_program(&program);
                }
            }
        }
    }

    pub fn run_program(&mut self, program: &Program) {
        for cmd in program {
            self.run_cmd(cmd);
        }
    }

    pub fn run_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Basic(outputs, updates, advance) => {
                for output in outputs {
                    self.output(self.eval_expr(output));
                }
                let writes: Vec<_> = updates
                    .iter()
                    .map(|(i, expr)| (i, self.eval_expr(expr)))
                    .collect();
                for (i, v) in writes {
                    self.memory[(self.head + i) as usize] = v;
                }
                self.head += *advance;
            }
            Stmt::Input(offset) => {
                self.input(*offset);
            }
            Stmt::Loop(bir) => {
                while self.memory[self.head as usize] != 0 {
                    self.run_bir(bir);
                }
            }
        }
    }

    pub fn eval_expr(&self, expr: &Expr) -> u8 {
        match expr {
            Expr::Add(offsets, k) => {
                let mut result = *k;
                for offset in offsets {
                    result = result.wrapping_add(self.memory[(self.head + *offset) as usize]);
                }
                result
            }
            Expr::Poly(_) => {
                todo!()
            }
        }
    }

    pub fn run_bir(&mut self, bir: &Vec<Stmt>) {
        for stmt in bir {
            self.run_stmt(stmt);
        }
    }

    fn input(&mut self, offset: isize) {
        match io::stdin().read(
            &mut self.memory[(self.head + offset) as usize..(self.head + offset + 1) as usize],
        ) {
            Ok(0) | Err(_) => {
                self.memory[(self.head + offset) as usize] = 0;
            }
            _ => {}
        }
    }

    fn output(&self, value: u8) {
        io::stdout().write(&[value]).unwrap();
    }
}
