// term is a command group for the terminal
// it contains settings and preferences for the terminal

use crate::traits::Command;
use crate::traits::CommandGroup;
use crate::traits::Config;
use serde_json::from_reader;
use std::fs::File;

pub fn init_group() -> CommandGroup {
    let mut term: CommandGroup = CommandGroup::new("term");
    // add commands here
    // eg: beed.add_command(Box::new(CommandName));

    term.add_command(Box::new(ListCommand));

    term
}

pub struct ListCommand;

impl Command for ListCommand {
    fn name(&self) -> &str {
        "list"
    }

    fn execute(&self, input: &str) -> String {
        let args: Vec<&str> = input.split_whitespace().collect();
        if args.len() > 1 {
            return format!("Error: too many arguments");
        }
        let file: File = File::open("C:\\Users\\ooich\\.vscode\\apps\\beedterminal2\\src\\settings.json").unwrap();
        let settings: Config = from_reader(file).unwrap();
        let mut output: String = String::from("Welcome to the BeedTerminal terminal interface.\nYou can manage the settings here.\n".to_string());
        output.push_str("Settings:\n");
        output.push_str(format!("Caret: {}\n", settings.caret).as_str());

        output
    }
}