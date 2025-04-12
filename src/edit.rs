mod zip;

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use color_print::{cprint, cprintln};
use tempfile::TempDir;
use crate::config::Config;
use crate::edit::zip::{unzip_from_bytes, zip_dir_to_bytes};
use crate::file_contents::FileContents;

pub fn edit<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    print!("Creating temporary directory... ");
    let start = Instant::now();
    let Ok(temp_dir) = TempDir::new() else { return Err("E05 Failed to create temp directory".to_owned()) };
    let Some(temp_dir_string) = temp_dir.path().to_str() else { return Err("E06 Failed get temp directory path".to_owned()) };
    let file_name = path.as_ref().file_stem().ok_or(format!("Invalid path: {:?}", path.as_ref()))?;
    let file_name = file_name.to_str().ok_or(format!("Invalid file name: {:?}", path.as_ref()))?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);

    let path_contents = FileContents::from_path(&path)?;

    if let Some(path_contents) = path_contents {
        print!("Extracting project... ");
        let start = Instant::now();
        unzip_from_bytes(path_contents.zipped_contents(), temp_dir.path())?;
        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
    }
    else {
        print!("Creating default project... ");
        let start = Instant::now();
        fs::write(temp_dir.path().join("Cargo.toml"), format!("[package]
name = \"{file_name}\"
version = \"0.1.0\"
edition = \"2024\"

[dependencies]
")).map_err(|e| format!("E09 Failed to create file: {}", e))?;
        fs::create_dir(temp_dir.path().join("src")).map_err(|e| format!("E10 Failed to create directory: {}", e))?;
        fs::write(temp_dir.path().join("src").join("main.rs"), "fn main() {
    println!(\"Hello, world!\");
}").map_err(|e| format!("E11 Failed to create file: {}", e))?;
        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
    }

    let binary = loop {
        println!("Opening editor... ");
        if let Err(e) = config.rust_project_edit_command().to_command(Some(temp_dir_string))?.output() {
            return Err(format!("Error when running project edit command: {}", e));
        }

        let args: &[&str] = if *config.use_debug_mode() {
            println!("Building binary (debug)... ");
            &["build"]
        } else {
            println!("Building binary (release)... ");
            &["build", "--release"]
        };

        let output = Command::new("cargo")
            .current_dir(temp_dir.path())
            .args(args)
            .status();
        let output = output.map_err(|e| format!("Error when running binary command: {}", e))?;

        if !output.success() {
            println!("Cargo build failed with code {:?}", output.code());
            println!("Reopen editor? (y/N): ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.to_ascii_lowercase().trim() != "y" {
                break None;
            }
        }
        else {
            let binary_path = temp_dir.path().join("target");

            let binary_path = if *config.use_debug_mode() {
                binary_path.join("debug")
            } else {
                binary_path.join("release")
            };

            #[cfg(unix)]
            let binary_path = binary_path.join(file_name);
            #[cfg(windows)]
            let binary_path = binary_path.join(format!("{file_name}.exe"));
            #[cfg(not(any(unix, windows)))]
            compile_error!("This crate only supports Unix or Windows targets.");

            print!("Reading binary... ");
            let start = Instant::now();
            let binary = fs::read(&binary_path).map_err(|e| format!("E12 Failed to read file {binary_path:?}: {}", e))?;
            let time = Instant::now() - start;
            cprintln!("<cyan>[{:?}]</>", time);
            break Some(binary);
        }
    };

    print!("Cleaning up target directory... ");
    let start = Instant::now();
    fs::remove_dir_all(temp_dir.path().join("target")).map_err(|e| format!("E35 Failed to remove temporary directory: {}", e))?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);

    print!("Zipping project... ");
    let start = Instant::now();
    let project_zip = zip_dir_to_bytes(temp_dir)?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);


    if binary.is_some() {
        cprint!("Writing rss file <green, bold>(project and binary)</>... ");
    }
    else {
        cprint!("Writing rss file <red, bold>(no binary)</>... ");
    }
    let file_contents = FileContents::new(project_zip, binary.unwrap_or(vec![]));
    let start = Instant::now();
    file_contents.save(path)?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);
    file_contents.print_stats();

    Ok(())
}