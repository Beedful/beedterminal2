use crate::path::{self, GLOBAL_PATH};
use crate::traits::{Command, Path};
// use std::borrow::Borrow;
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

        // let dir: &str = match args.get(0) {
        //     Some(arg) => arg,
        //     None => &path::GLOBAL_PATH.read().unwrap().cwd as &str,
        // };

        let cwd: std::path::PathBuf = std::env::current_dir().unwrap();

        let mut output: String = String::new();
        for entry in fs::read_dir(format!("{}", cwd.display())).unwrap() {
            let entry = entry.unwrap();
            output.push_str(&format!("{}\n", entry.file_name().to_string_lossy()));
        }

        output
    }

    fn name(&self) -> &str {
        "ls"
    }
}

pub struct CdCommand;
impl Command for CdCommand {
    fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() > 1 {
            return format!("Error: too many arguments");
        }

        let new_cwd: &str = args[0];
        match std::env::set_current_dir(new_cwd) {
            Ok(_) => {
                GLOBAL_PATH.write().unwrap().cwd = new_cwd.to_string();
                "".to_string()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    fn name(&self) -> &str {
        "cd"
    }
}