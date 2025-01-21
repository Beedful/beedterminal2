mod traits;
mod commands;
mod path;

use std::io::Write;
use crate::traits::Command;
use std::time::Instant;
// use std::fs;
// use crate::commands::system::*;
// use crate::commands::utility::*;

fn main() {
    let mut commands: Vec<Box<dyn Command>> = vec![];
    let mut command_groups: Vec<traits::CommandGroup> = vec![];
    let mut cmd_groups_foradding: Vec<traits::CommandGroup> = vec![];
    let beed_group: traits::CommandGroup = commands::beed::init_group();
    let beed_group_foradding: traits::CommandGroup = commands::beed::init_group();
    let term_group: traits::CommandGroup = commands::term::init_group();
    let term_group_foradding: traits::CommandGroup = commands::term::init_group();

    command_groups.push(beed_group);
    command_groups.push(term_group);
    cmd_groups_foradding.push(beed_group_foradding);
    cmd_groups_foradding.push(term_group_foradding);

    for command in commands::system::commands() {
        commands.push(command);
    }

    for command in commands::utility::commands() {
        commands.push(command); 
    }

    for cmd_group in cmd_groups_foradding {
        for cmd in cmd_group.commands {
            commands.push(cmd);
        }
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

        'cmd_loop: for cmd in &commands {
            for cmd_group in &command_groups {
                if cmd_group.commands.iter().any(|c| c.name() == *cmd_name) {
                    break 'cmd_loop;
                }
            }

            if let Some(group) = command_groups.iter().find(|group| group.name() == args[0]) {
                if args.len() > 1 {
                    let sub_command = args[1];
                    if let Some(command) = group.commands.iter().find(|c| c.name() == sub_command) {
			let time_start = Instant::now(); 
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
		let time_start_out = Instant::now();
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
