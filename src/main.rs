use std::io::{stdout, Write};

mod expr;
mod meta_operation;

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("cannot read line");
        if let Ok(op) = meta_operation::parse(&line) {
            op.execute()
        } else if let Ok(expr) = expr::parse::expr(&line) {
            if let Err(_) = expr.typecheck() {
                eprintln!("! type error: ");
            } else {
                expr.eval();
            }
        } else {
            eprintln!("! invalid input");
        }
    }
}
