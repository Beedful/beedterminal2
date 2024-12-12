mod traits;
mod commands;
mod path;

use std::io::Write;
use crate::traits::Command;
use std::fs;
// use crate::commands::system::*;
// use crate::commands::utility::*;

fn main() {
    let mut commands: Vec<Box<dyn Command>> = vec![];

    for command in commands::system::commands() {
        commands.push(command);
    }

    for command in commands::utility::commands() {
        commands.push(command);
    }

    println!("Welcome to BeedTerminal 2");
    println!("Type 'exit' to exit");
    let mut input: String = String::new();

    while input.trim() != "exit" {
        input.clear();
        let cwd: std::path::PathBuf = std::env::current_dir().unwrap();
        print!("{} $ ", cwd.display());
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let args: Vec<&str> = input.trim().split_whitespace().collect();

        let cmd_name: &&str = args.get(0).unwrap_or(&"");

        if input.trim() == "exit" {
            break;
        }

        for cmd in &commands {
            if cmd.name() == *cmd_name {
                let output = cmd.execute(&args[1..].join(" "));
                println!("{}", output);
                break;
            }
        }
    }

    println!("Bye!");
}
