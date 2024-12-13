// use crate::path;
use crate::traits::Command;

pub fn commands() -> Vec<Box<dyn Command>> {
    vec![Box::new(HelpCommand), Box::new(ClearCommand)]
}

pub struct HelpCommand;
impl Command for HelpCommand {
    fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() > 1 {
            return format!("Error: too many arguments");
        }

        let output: String = String::from("BeedTerminal 2\nView the repository for more info: https://github.com/Beedful/beedterminal2");
        output
    }

    fn name(&self) -> &str {
        "help"
    }
}

pub struct ClearCommand;
impl Command for ClearCommand {
    fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() > 1 {
            return format!("Error: too many arguments");
        }
        println!("\x1B[2J\x1B[1;1H");
        "".to_string()
    }

    fn name(&self) -> &str {
        "clear"
    }
}