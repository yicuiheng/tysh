use std::{collections::HashMap, fmt};

use super::{Type, TypedOperation};

pub struct CommandOperation {
    pub command_name: String,
    pub args: HashMap<String, String>,
}

impl CommandOperation {
    pub fn execute(&self) {
        todo!();
    }
}

impl fmt::Display for CommandOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args: Vec<String> = self
            .args
            .iter()
            .map(|(key, val)| format!("--{}:{}", key, val))
            .collect();
        write!(f, "{} {}", self.command_name, args.join(" "))
    }
}

use once_cell::sync::Lazy;
static TYPE_DEFINITIONS: Lazy<HashMap<String, Type>> = Lazy::new(|| {
    let mut buf = String::new();
    use std::io::Read;
    std::fs::File::open("./type-definitions") // TODO:
        .expect("type definition file not found")
        .read_to_string(&mut buf)
        .expect("cannot read type definitions");
    let mut result = HashMap::new();
    for line in buf.lines() {
        let mut splitter = line.splitn(2, ':');
        let name = splitter.next().unwrap().to_string();
        let typ = splitter.next().unwrap().to_string();
        result.insert(name, typ);
    }
    result
});

impl TypedOperation for CommandOperation {
    fn typ(&self) -> &Type {
        TYPE_DEFINITIONS.get(&self.command_name).unwrap() // TODO: not to use `unwrap`
    }
}
