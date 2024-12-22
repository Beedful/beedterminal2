use crate::path::GLOBAL_PATH;
use crate::traits::Command;
// use std::borrow::Borrow;
use std::fs;

pub fn commands() -> Vec<Box<dyn Command>> {
    vec![Box::new(LsCommand), Box::new(CdCommand), Box::new(GetcwdCommand), Box::new(MkdirCommand), Box::new(TouchCommand)]
}

pub struct LsCommand;
impl Command for LsCommand {
    fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() > 1 {
            return format!("Error: too many arguments");
        }

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

pub struct GetcwdCommand;
impl Command for GetcwdCommand {
    fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() > 1 {
            return format!("Error: too many arguments");
        }

        let cwd: std::path::PathBuf = std::env::current_dir().unwrap();
        cwd.to_string_lossy().to_string()
    }    

    fn name(&self) -> &str {
        "cwd"
    }
}
pub struct MkdirCommand;
impl Command for MkdirCommand {
    fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() > 1 {
            return format!("Error: too many arguments");
        }
        let dir_path: &str = args[0];
        match fs::create_dir(dir_path) {
            Ok(_) => format!("Directory {} created", dir_path),
            Err(e) => format!("Error: {}", e),
        }
    }    

    fn name(&self) -> &str {
        "mkdir"
    }
}

pub struct TouchCommand;
impl Command for TouchCommand {
    fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() > 1 {
            return format!("Error: too many arguments");
        }
        
        match fs::File::create(input) {
            Ok(_) => format!("File {} created", input),
            Err(e) => format!("Error: {}", e),
        }

    }

    fn name(&self) -> &str {
        "touch"
    }
}