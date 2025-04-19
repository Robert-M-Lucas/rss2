Stores all the files and the built binary of a Rust project in a single file allowing similar usage to Python scripts.

## Installation
Install with
```bash
cargo install rs-script
```

### Config
> A large number of commands will not work if you don't have an editor configured in the config file!

Find the config file
```bash
rss config -w
```
> You will need to manually edit this file if you don't have the default editor 

Modify the config with
```bash
rss config
```
> This uses the editor configured in the config file

Reset the config file
```bash
rss config -r
```

## Editing
Create/edit a file with
```bash
rss edit myfile.rss
```

## Running
Run a file with
```bash
rss run myfile.rss
```

## Other Commands
Remove the compiled binary part of a file with
```bash
rss strip myfile.rss
```

Recompile a file with
```bash
rss recompile myfile.rss
```

## Cross Compatability
Files save what platform a binary was compiled for. An error and suggested remedy will be displayed if the current binary attached to a file was compiled for a different platform.