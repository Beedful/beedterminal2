use crate::traits::Command;
use std::fs;

pub fn commands() -> Vec<Box<dyn Command>> {
    vec![Box::new(LsCommand), Box::new(CdCommand)]
}

pub struct LsCommand;
impl Command for LsCommand {
    fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() > 1 {
            return format!("Error: too many arguments");
        }

        let dir = match args.get(0) {
            Some(arg) => arg,
            None => ".",
        };

        match fs::read_dir(dir) {
            Ok(entries) => {
                let mut output = String::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        output.push_str(&entry.file_name().to_string_lossy());
                        output.push('\n');
                    }
                }
                output
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    fn name(&self) -> &str {
        "ls"
    }
}

pub struct CdCommand;
impl Command for CdCommand {
    fn execute(&self, input: &str) -> String {
        ("Command is still in development").to_string()
    }

    fn name(&self) -> &str {
        "cd"
    }
}