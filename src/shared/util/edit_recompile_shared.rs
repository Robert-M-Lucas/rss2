use crate::shared::config::{Config, get_config_path};
use crate::shared::util::file_contents::FileContents;
use crate::shared::util::zip::unzip_from_bytes;
use crate::time;
use color_print::cprintln;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

pub fn create_temp_project_dir<P: AsRef<Path>>(
    path: P,
) -> Result<(TempDir, String, String), String> {
    let temp_dir = time!(
        "Creating temporary directory",
        false,
        TempDir::new().map_err(|e| format!("E05 Failed to create temp directory: {e}"))?
    );

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

    Ok((temp_dir, temp_dir_string, file_name))
}

pub fn extract_project<P: AsRef<Path>>(
    path_contents: &FileContents,
    temp_dir: P,
) -> Result<(), String> {
    time!(
        "Extracting project",
        false,
        unzip_from_bytes(path_contents.zipped_contents(), temp_dir.as_ref())?;
    );
    Ok(())
}

pub fn project_edit_loop<P: AsRef<Path>>(
    mut skip_first: bool,
    compile_binary: bool,
    config: &Config,
    temp_dir: P,
    temp_dir_string: &str,
    file_name: &str,
) -> Result<Option<Vec<u8>>, String> {
    // TODO: Allow user to choose bin with bin-choice.txt file

    Ok(loop {
        if !skip_first {
            println!("Opening editor (and waiting for close)... ");

            if let Err(e) = config
                .rust_project_edit_command_blocking()
                .run_command(Some(temp_dir_string))?
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

        if !compile_binary {
            cprintln!("<yellow, bold>Not compiling binary due to config</>");
            return Ok(None);
        }

        let args: &[&str] = if config.use_debug_mode() {
            println!("Building binary (debug)... ");
            &["build"]
        } else {
            println!("Building binary (release)... ");
            &["build", "--release"]
        };

        let output = Command::new("cargo")
            .current_dir(temp_dir.as_ref())
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
            let binary_path = temp_dir.as_ref().join("target");

            let binary_path = if config.use_debug_mode() {
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

            let binary = time!("Reading built binary", false, fs::read(&binary_path));

            let binary = match binary {
                Err(e) => {
                    cprintln!(
                        "<red, bold>Failed to find built binary at path {:?}</> ({e})",
                        binary_path
                    );
                    cprintln!("<cyan, bold>Is a binary with a different name being built?</>");
                    println!("Reopen editor? (y/N): ");
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    if input.to_ascii_lowercase().trim() != "y" {
                        break None;
                    }
                    continue;
                }
                Ok(b) => b,
            };

            break Some(binary);
        }
    })
}
