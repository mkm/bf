use crate::bir::{Expr, Stmt};
use std::collections::HashMap;
use std::mem;

#[derive(Debug)]
struct StableState {
    outputs: Vec<Expr>,
    updates: HashMap<isize, Expr>,
    advance: isize,
}

impl StableState {
    fn new() -> Self {
        Self {
            outputs: Vec::new(),
            updates: HashMap::new(),
            advance: 0,
        }
    }

    fn output(&mut self, new_outputs: Vec<Expr>) {
        for output in new_outputs {
            self.outputs.push(self.eval(output));
        }
    }

    fn update(&mut self, new_updates: HashMap<isize, Expr>) {
        let assignments: Vec<_> = new_updates
            .into_iter()
            .map(|(i, v)| (i + self.advance, self.eval(v)))
            .collect();
        for (i, v) in assignments {
            self.updates.insert(i, v);
        }
    }

    fn advance(&mut self, offset: isize) {
        self.advance += offset;
    }

    fn resolve(&mut self, result: &mut Vec<Stmt>) {
        let outputs = mem::take(&mut self.outputs);
        let updates = mem::take(&mut self.updates);
        let advance = mem::take(&mut self.advance);
        if !outputs.is_empty() || !updates.is_empty() || advance != 0 {
            result.push(Stmt::Basic(outputs, updates, advance));
        }
    }

    fn eval(&self, expr: Expr) -> Expr {
        match expr {
            Expr::Add(offsets, k) => {
                let mut new_offsets = Vec::new();
                let mut new_k = k;
                for offset in offsets {
                    let local_offset = offset + self.advance;
                    match self.updates.get(&local_offset) {
                        Some(Expr::Add(offsets0, k0)) => {
                            for offset0 in offsets0 {
                                new_offsets.push(*offset0);
                            }
                            new_k = new_k.wrapping_add(*k0);
                        }
                        Some(Expr::Poly(_)) => {
                            todo!()
                        }
                        None => {
                            new_offsets.push(local_offset);
                        }
                    }
                }
                Expr::Add(new_offsets, new_k)
            }
            Expr::Poly(_) => {
                todo!()
            }
        }
    }
}

pub fn opt_bir(bir: Vec<Stmt>) -> Vec<Stmt> {
    let mut state = StableState::new();
    let mut result = Vec::new();

    for stmt in bir {
        match stmt {
            Stmt::Basic(outputs, updates, advance) => {
                state.output(outputs);
                state.update(updates);
                state.advance(advance);
            }
            Stmt::Input(offset) => {
                state.resolve(&mut result);
                result.push(Stmt::Input(offset));
            }
            Stmt::Loop(body) => {
                let new_body = opt_bir(body);
                if new_body.len() == 1 {
                    match &new_body[0] {
                        Stmt::Basic(outputs, updates, advance) => {
                            if let Some(Expr::Add(offsets, k)) = updates.get(&0) {
                                if outputs.is_empty()
                                    && offsets.len() == 1
                                    && offsets[0] == 0
                                    && *k == 255
                                    && updates.len() == 1
                                    && *advance == 0
                                {
                                    state.update([(0, Expr::Add(Vec::new(), 0))].into());
                                    continue;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                state.resolve(&mut result);
                result.push(Stmt::Loop(new_body));
            }
        }
    }

    state.resolve(&mut result);

    result
}
