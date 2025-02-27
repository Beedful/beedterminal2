# BeedTerminal 2
This is the repository for BeedTerminal 2, a terminal app.

## Prequisites
You must have Rust and `cargo` installed. You can install Rust [here](https://www.rust-lang.org/tools/install).

## Usage
After downloading or cloning this repository, navigate to the directory and run the following command:

```
cargo run
```

After some compilation, the terminal should run. Try using the `ls` command!

## Command list
| Command | Use | Format | Type |
| --- | --- | --- | --- |
| `help` | Get help. | `help` | Utlility |
| `ls` | List directory contents. | `ls` | System |
| `cd` | Change working directory. | `cd <directory>` | System |
| `clear` | Clear terminal. | `clear` | Utility |
| `cwd` | Get working directory. | `cwd` | System |
| `mkdir` | Make a new directory/folder. | `mkdir <directory name>` | System |
| `touch` | Make a new file. | `touch <filename>` | System |
| `cat` | Read a file's contents. | `cat <filename>` | System |

## Reasons
These are the reasons I created this terminal from scratch.
1. **Good Learning**: This terminal is built in Rust, and I can learn a lot of Rust from this terminal.
2. **Exploring Systems Programming**: By writing a terminal, I can learn to access the filesystem and explore systems programming, starting from the filesystem.

## Why you should use it (once its fully stable)
You should use this once its fully stable. Reasons:

1. **Blazingly Fast**: This terminal is written in Rust. It uses the standard Rust library, resulting in extremely fast operations.
2. **Simple**: This terminal eliminates the complexity and messiness of typical terminals.
3. **Open Source**: Who doesn't love open source?
4. **Minimal**: This terminal doesn't support cool but useless features like other terminals. It is designed to give a minimal experience to the user.

## How it works
This section describes how the command system works. First, let's explore the structure.
1. `main.rs`: This handles the command output and displaying the input (`~ $`).
2. `traits.rs`: This has all the traits which can be used when creating other stuff such as new commands.
3. `path.rs`: This manages the working directory.
4. `commands/`: This directory holds all the commands, conventions and command helpers.
5. `commands/conventions.md`: This describes the conventions for standardizing commands.
6. `commands/mod.rs`: This exports the commands.
7. (all other `.rs` files in `commands/`): Describes commands. Commands are split into categories and each category has its own file. (utility, system, ssh etc)

A command is first created in its own category file. It has the `execute()` and `name()` functions, which execute the command and show the name of the command respectively. The `execute()` command returns a `String` output, which is then printed in `main.rs`.

The `main.rs` file works by the following:
1. Receive user input
2. Match input to command
3. Receive the output (the return value of the command) and prints it
4. Repeat
