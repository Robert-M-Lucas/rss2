# RS-Script

Stores all the files and the built binary of a Rust project in a single, runnable file allowing similar usage to Python scripts.

## Installation
```bash
cargo install rs-script
```

## Config
_A large number of commands will not work if you don't have an editor configured in the config file!_

- Find the config file: `rss config -w`/`rss conf -w`

You will need to manually edit this file if you don't have the default editor

- Modify the config with: `rss config`/`rss conf`

This uses the editor configured in the config file (defaults to vscode).

Example nvim configuration (Linux):
```json
"rust_project_edit_command_blocking": {
    "command": "nvim",
    "args": [
      "$dir$"
    ],
    "inherit_shell": true
}
```

Example RustRover configuration (Linux):
```json
"rust_project_edit_command_blocking": {
    "command": "rust-rover",
    "args": [
      "$dir$"
    ],
    "inherit_shell": false
}
```

- Reset the config file: `rss config -r`/`rss conf -r`

## Editing
```
rss edit myfile.rss [OR] rss e myfile.rss
```

A `cr-origin.sh/cr-origin.cmd` script will automatically be created allowing you to `cargo run` in the original directory for ease of development.

## Running
```
rss run myfile.rss [OR] rss r myfile.rss
```

### Cross Compatability
The platform a binary was compiled for is automatically saved. Should this not match the current platform, the file will be automatically recompiled.

## Other Commands
Add `-v` after `rss` to get verbose information.
- Edit an rss file and fail if it already exists: `rss new myfile.rss`/`rss n myfile.rss`
- Remove the compiled binary part of a file (useful for sharing): `rss strip myfile.rss`/`rss s myfile.rss`
- Recompile a file: `rss recompile myfile.rss`/`rss rcmp myfile.rss`
- Extract the Rust source from an rss file: `rss extract myfile.rss`/`rss ext myfile.rss`
- Create an rss file from an existing rust project: `rss pack project_dir`/`rss p project_dir`
- Print statistics for an rss file: `rss stats myfile.rss`/`rss stat myfile.rss`
- Print the file tree within an rss file: `rss tree myfile.rss`/`rss t myfile.rss`
- Print the contents of a file / files within an rss file (by default only shows .rs files): `rss cat myfile.rss`/`rss c myfile.rss`
- Read this README: `rss readme`
- Command help (useful for learning command shorthands): `rss help`

## Linux Only
On Linux, `rss` files are automatically made executable using the `#!/usr/bin/env rss-run` 
shebang. With `rss-run` in your path, you can run `rss` files like any executable:
```bash
./myfile.rss
```