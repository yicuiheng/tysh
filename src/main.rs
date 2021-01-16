use std::io::{stdout, Write};

mod operation;
fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("cannot read line");
        match operation::parse(&line) {
            Ok(op) => op.execute(),
            Err(msg) => eprintln!("! {}", msg),
        }
    }
}
