use std::collections::HashMap;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    command: String,
    key: String,
}
struct TodoList {
    // true = to do, false = done
    items: HashMap<String, bool>,
}

impl TodoList {
    fn new() -> TodoList {
        let items: HashMap<String, bool> =
            HashMap::<String, bool>::new();

        TodoList { items: items }
    }
}

fn main() {
    let args = Cli::parse();

    println!("Command line: {} {}", args.command, args.key);
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_todo() {
        let todo = TodoList::new();
    }
}