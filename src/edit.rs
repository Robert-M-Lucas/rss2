mod zip;

use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;
use crate::config::Config;
use crate::edit::zip::{unzip_from_bytes, zip_dir_to_bytes};
use crate::file_contents::FileContents;

pub fn edit<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    println!("Creating temporary directory");
    let Ok(temp_dir) = TempDir::new() else { return Err("E05 Failed to create temp directory".to_owned()) };
    let Some(temp_dir_string) = temp_dir.path().to_str() else { return Err("E06 Failed get temp directory path".to_owned()) };
    let file_name = path.as_ref().file_stem().ok_or(format!("Invalid path: {:?}", path.as_ref()))?;
    let file_name = file_name.to_str().ok_or(format!("Invalid file name: {:?}", path.as_ref()))?;

    let path_contents = FileContents::from_path(&path)?;

    if let Some(path_contents) = path_contents {
        println!("Extracting project");
        unzip_from_bytes(path_contents.zipped_contents(), temp_dir.path())?;
    }
    else {
        println!("Creating default project");
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
    }

    let binary = loop {
        println!("Opening editor");
        if let Err(e) = config.rust_project_edit_command().to_command(Some(temp_dir_string))?.output() {
            return Err(format!("Error when running project edit command: {}", e));
        }

        println!("Building binary");
        let output = Command::new("cargo")
            .current_dir(temp_dir.path())
            .arg("build")
            .output();
        let output = output.map_err(|e| format!("Error when running binary command: {}", e))?;

        if !output.status.success() {
            println!("Cargo build failed with code {:?}", output.status.code());
            println!("Reopen editor? (y/N): ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.to_ascii_lowercase().trim() != "y" {
                break None;
            }
        }
        else {
            let binary_path = temp_dir.path().join("target").join("debug");
            #[cfg(unix)]
            let binary_path = binary_path.join(file_name);
            #[cfg(windows)]
            let binary_path = binary_path.join(format!("{file_name}.exe"));
            #[cfg(not(any(unix, windows)))]
            compile_error!("This crate only supports Unix or Windows targets.");

            let binary = fs::read(&binary_path).map_err(|e| format!("E12 Failed to read file {binary_path:?}: {}", e))?;
            break Some(binary);
        }
    };

    fs::remove_dir_all(temp_dir.path().join("target")).map_err(|e| format!("E35 Failed to remove temporary directory: {}", e))?;

    println!("Zipping project");
    let project_zip = zip_dir_to_bytes(temp_dir)?;

    if binary.is_some() {
        println!("Writing rss file (project and binary)");
    }
    else {
        println!("Writing rss file (no binary)");
    }
    let file_contents = FileContents::new(project_zip, binary.unwrap_or(vec![]));
    file_contents.save(path)?;

    Ok(())
}