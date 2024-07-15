// src/main.rs

use clap::Parser;
use todo_cli::TodoList;

#[derive(Parser)]
struct Cli {
    command: String,
    key: String,
}

fn main() {
    let args = Cli::parse();

    println!("Command line: {} {}", args.command, args.key);
}
