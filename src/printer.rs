use crate::bir::{Expr, Stmt};

pub fn print_expr(expr: &Expr) {
    match expr {
        Expr::Add(offsets, k) => {
            for offset in offsets {
                print!("${offset}+");
            }
            print!("{k}");
        }
        Expr::Poly(_) => {
            todo!()
        }
    }
}

pub fn print_stmt(stmt: &Stmt) {
    match stmt {
        Stmt::Basic(outputs, updates, advance) => {
            let mut sep = false;
            print!("{{");
            for output in outputs {
                if sep {
                    print!(" | ");
                }
                sep = true;
                print_expr(output);
            }
            for (i, expr) in updates {
                if sep {
                    print!(" | ");
                }
                sep = true;
                print!("${i}:=");
                print_expr(expr);
            }
            if *advance != 0 {
                if sep {
                    print!(" | ");
                }
                if *advance > 0 {
                    print!(">{}", advance);
                } else {
                    print!("<{}", -advance);
                }
            }
            print!("}}");
        }
        Stmt::Input(offset) => {
            print!(",{offset}");
        }
        Stmt::Loop(bir) => {
            print!("[");
            print_bir(bir);
            print!("]");
        }
    }
}

pub fn print_bir(bir: &Vec<Stmt>) {
    for stmt in bir {
        print_stmt(stmt);
    }
}
