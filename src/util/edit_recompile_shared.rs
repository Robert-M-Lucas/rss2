use crate::config::{Config, get_config_path};
use crate::util::file_contents::FileContents;
use crate::util::zip::unzip_from_bytes;
use color_print::cprintln;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use tempfile::TempDir;

pub fn create_temp_project_dir<P: AsRef<Path>>(
    path: P,
) -> Result<(TempDir, String, String), String> {
    print!("Creating temporary directory... ");
    let start = Instant::now();
    let Ok(temp_dir) = TempDir::new() else {
        return Err("E05 Failed to create temp directory".to_owned());
    };
    let Some(temp_dir_string) = temp_dir.path().to_str().map(|s| s.to_owned()) else {
        return Err("E06 Failed get temp directory path".to_owned());
    };
    let file_name = path
        .as_ref()
        .file_stem()
        .ok_or(format!("Invalid path: {:?}", path.as_ref()))?;
    let file_name = file_name
        .to_str()
        .ok_or(format!("Invalid file name: {:?}", path.as_ref()))?
        .to_owned();
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);
    Ok((temp_dir, temp_dir_string, file_name))
}

pub fn extract_project(path_contents: &FileContents, temp_dir: &TempDir) -> Result<(), String> {
    print!("Extracting project... ");
    let start = Instant::now();
    unzip_from_bytes(path_contents.zipped_contents(), temp_dir.path())?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);
    Ok(())
}

pub fn project_edit_loop(
    mut skip_first: bool,
    config: &Config,
    temp_dir: &TempDir,
    temp_dir_string: &str,
    file_name: &str,
) -> Result<Option<Vec<u8>>, String> {
    Ok(loop {
        if !skip_first {
            println!("Opening editor... ");
            if let Err(e) = config
                .rust_project_edit_command()
                .to_command(Some(temp_dir_string))?
                .output()
            {
                return Err(format!(
                    "E49 Error when running project edit command: {}\n\
                    Check/edit the command used in '{}'.\n  - \
                    If you have your config edit program correctly configured use `rss config` to modify the config",
                    e,
                    get_config_path()?.as_os_str().to_string_lossy()
                ));
            }
        }
        skip_first = false;

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
        } else {
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

            print!("Reading built binary... ");
            let start = Instant::now();
            let binary = fs::read(&binary_path)
                .map_err(|e| format!("E12 Failed to read file {binary_path:?}: {}", e))?;
            let time = Instant::now() - start;
            cprintln!("<cyan>[{:?}]</>", time);
            break Some(binary);
        }
    })
}
