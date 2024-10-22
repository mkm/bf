use crate::syntax::{Cmd, Program};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr {
    Add(Vec<isize>, u8),
    Poly(HashMap<Vec<isize>, u8>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Basic(Vec<Expr>, HashMap<isize, Expr>, isize),
    Input(isize),
    Loop(Vec<Stmt>),
}

impl Stmt {
    pub fn from_cmd(cmd: Cmd) -> Self {
        match cmd {
            Cmd::Inc => Stmt::Basic(Vec::new(), [(0, Expr::Add(vec![0], 1))].into(), 0),
            Cmd::Dec => Stmt::Basic(Vec::new(), [(0, Expr::Add(vec![0], u8::MAX))].into(), 0),
            Cmd::Fwd => Stmt::Basic(Vec::new(), HashMap::new(), 1),
            Cmd::Bwd => Stmt::Basic(Vec::new(), HashMap::new(), -1),
            Cmd::In => Stmt::Input(0),
            Cmd::Out => Stmt::Basic(vec![Expr::Add(vec![0], 0)], HashMap::new(), 0),
            Cmd::Loop(program) => Stmt::Loop(Self::from_program(program)),
        }
    }

    pub fn from_program(program: Program) -> Vec<Self> {
        program.into_iter().map(Self::from_cmd).collect()
    }
}
