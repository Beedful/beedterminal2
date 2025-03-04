mod traits;
mod commands;

use std::io::Write;
use crate::traits::Command;
use std::time::Instant;
// use std::fs;
// use crate::commands::system::*;
// use crate::commands::utility::*;

fn main() {
    // define variables
    let mut commands: Vec<Box<dyn Command>> = vec![]; // commands list
    let mut command_groups: Vec<&traits::CommandGroup> = vec![]; // commands group list
    let mut cmd_groups_foradding: Vec<&traits::CommandGroup> = vec![];
    let beed_group: traits::CommandGroup = commands::beed::init_group();
    let term_group: traits::CommandGroup = commands::term::init_group();

    command_groups.push(&beed_group);
    command_groups.push(&term_group);
    cmd_groups_foradding.push(&beed_group);
    cmd_groups_foradding.push(&term_group);

    // add system module commmands
    for command in commands::system::commands() {
        commands.push(command);
    }

    // add utility module commands
    for command in commands::utility::commands() {
        commands.push(command); 
    }

    // add command groups' commands
    for cmd_group in cmd_groups_foradding {
        for cmd in &cmd_group.commands {
            commands.push(Box::new((**cmd).clone())); // FIX: fix types problem
        }
    }

    println!("Welcome to BeedTerminal 2");
    println!("Type 'exit' to exit");
    let mut input: String = String::new(); // input string

    while input.trim() != "exit" {
        input.clear();
        let cwd: std::path::PathBuf = std::env::current_dir().unwrap(); // get cwd to print
        print!("{} $ ", cwd.display()); // print command prompt
        std::io::stdout().flush().unwrap(); // flush stdout
        std::io::stdin().read_line(&mut input).unwrap(); // read input
        let args: Vec<&str> = input.trim().split_whitespace().collect(); // take arguments

        let cmd_name: &&str = args.get(0).unwrap_or(&"");

        // take exit command
        if input.trim() == "exit" {
            break;
        }

        'cmd_loop: for cmd in &commands {
            for cmd_group in &command_groups {
                if cmd_group.commands.iter().any(|c| c.name() == *cmd_name) {
                    break 'cmd_loop;
                }
            }

            if let Some(group) = command_groups.iter().find(|group| group.name() == args[0]) {
                if args.len() > 1 {
                    let sub_command: &str = args[1];
                    if let Some(command) = group.commands.iter().find(|c| c.name() == sub_command) {
			            let time_start: Instant = Instant::now(); 
			            let output: String = command.execute(&args[2..].join(" "));
                        if output == "" {
				            println!("{:.2?}", time_start.elapsed().as_micros());
                            print!("");
                            break;
                        }
                        println!("{}", output);
			            let time_end: u128 = time_start.elapsed().as_micros();
			            println!("{:.2?}", time_end);
                        break;
                    }
                } else {
                    println!("Please specify a sub-command");
                }
            }

            if cmd.name() == *cmd_name {
                // check if its a subcommand of a command group
		        let time_start_out: Instant = Instant::now();
                let output: String = cmd.execute(&args[1..].join(" "));
                if output == "" {
			        println!("{:.2}", time_start_out.elapsed().as_micros());
                    print!("");
                    break;
                }
                println!("{}", output);
		        let time_end_out: u128 = time_start_out.elapsed().as_micros();
		        println!("{:.2?}", time_end_out);
                break;
            }
        }
    }

    println!("Bye!");
}
