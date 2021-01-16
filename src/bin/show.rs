use std::{io::Read, process::exit};

fn main() {
    let arg = std::env::args().nth(1).expect("filename required");

    const PREFIX: &str = "--filename:";
    if arg.starts_with(PREFIX) {
        let mut buf = String::new();
        let path = std::path::Path::new(&arg[PREFIX.len()..]);
        std::fs::File::open(path)
            .expect("file not found")
            .read_to_string(&mut buf)
            .expect("cannot read");
        println!("{}", buf);
    } else {
        eprintln!("arg must start with `--filename:`");
        exit(-1);
    }
}
