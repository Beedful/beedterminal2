# Command Conventions
This MD file lists the conventions that all commands **must** follow.

## Flags and Arguments

* Flags and arguments should have a prefix hyphen (`-`) to distinguish them from command names.
* Example: `ls -a` or `cd -p`

## Command Names

* Command names should be short and descriptive.
* Example: `ls`, `cd`, `mkdir`

## Command Structure

* Commands should follow the structure: `command [flags] [arguments]`
* Example: `ls -l -a /path/to/directory`

## Flag Conventions

* Single-character flags should be prefixed with a single hyphen (`-`).
* Example: `-a`, `-l`
* Multi-character flags should be prefixed with a double hyphen (`--`).
* Example: `--all`, `--long`

## Argument Conventions

* Arguments should be separated from flags by a space.
* Example: `cd /path/to/directory`
* Arguments can be optional or required, depending on the command.