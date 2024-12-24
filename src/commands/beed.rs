// use crate::path::GLOBAL_PATH;
use crate::traits::Command;
use crate::traits::CommandGroup;
// use std::borrow::Borrow;

pub fn init_group() -> CommandGroup {
    let mut beed: CommandGroup = CommandGroup::new("beed");
    // add commands here
    // eg: beed.add_command(Box::new(CommandName));

    beed.add_command(Box::new(InfoCommand));

    beed
}

pub struct InfoCommand;
impl Command for InfoCommand {
    fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() > 1 {
            return format!("Error: too many arguments");
        }

        let output: String = String::from("Welcome to the Beed interface!\n".to_string());

        output
    }

    fn name(&self) -> &str {
        "info"
    }
}


