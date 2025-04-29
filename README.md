# RS-Script

> !! Changes before 1.0.0 may be breaking !!

Stores all the files and the built binary of a Rust project in a single file allowing similar usage to Python scripts.

## Installation
```bash
cargo install rs-script
```

## Config
A large number of commands will not work if you don't have an editor configured in the config file!

- Find the config file: `rss config -w`

You will need to manually edit this file if you don't have the default editor 

- Modify the config with: `rss config`

This uses the editor configured in the config file

Example nvim configuration:
```json
"rust_project_edit_command_blocking": {
    "command": "nvim",
    "args": [
      "$$$"
    ],
    "inherit_shell": true
}
```

- Reset the config file: `rss config -r`

## Editing
```bash
rss edit myfile.rss
```

A `cr-origin.sh/cmd` script will automatically be created allowing you to `cargo run` in the original directory for ease of development.

## Running
```bash
rss run myfile.rss
```

## Cross Compatability
The platform a binary was compiled for is automatically saved. Should this not match the current platform, the file will be automatically recompiled.

## Other Commands
- Remove the compiled binary part of a file: `rss strip myfile.rss`
- Recompile a file: `rss recompile myfile.rss`
- Extract the Rust source from an rss file: `rss extract myfile.rss`
- Read this README: `rss readme`
- Command help: `rss help`