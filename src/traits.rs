use std::clone;
use serde::{Deserialize, Serialize};

pub trait Command {
    fn execute(&self, input: &str) -> String;
    fn name(&self) -> &str;
}

pub struct CommandGroup {
    pub name: String,
    pub commands: Vec<Box<dyn Command>>
}

impl CommandGroup {
    pub fn new(name: &str) -> Self {
        CommandGroup {
            name: name.to_string(),
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }
    pub fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() < 2 {
            return format!("Error: invalid number of arguments");
        }

        let command_name: &str = args[1];
        for command in &self.commands {
            if command.name() == command_name {
                return command.execute(&args[2..].join(" "));
            }
        }

        format!("Error: unknown command '{}'", command_name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub caret: String,
}
