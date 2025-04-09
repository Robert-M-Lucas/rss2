use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::{NamedTempFile, SpooledTempFile};
use crate::config::Config;
use crate::file_contents::FileContents;

pub fn run<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    println!("Reading binary from file");
    let path_contents = FileContents::from_path(&path)?.ok_or(format!("E36 File contents not found: {:?}", path.as_ref()))?;

    let temp_exe = NamedTempFile::new().map_err(|e| format!("E37 Temp file creation error: {:?}", e))?;

    println!("Writing binary to temporary file");
    fs::write(temp_exe.path(), path_contents.bin_contents()).map_err(|e| format!("E38 Temp file creation error: {:?}", e))?;

    #[cfg(unix)]
    {
        println!("Making temporary file executable (chmod)");
        Command::new("chmod").args([OsStr::new("+x"), temp_exe.path().as_os_str()]).status().map_err(|e| format!("E40 Failed to run chmod: {:?}", e))?;
    }

    let temp_exe_path = temp_exe.path().to_owned();
    temp_exe.keep().map_err(|e| format!("E41 Failed to mark binary as non-temporary {:?}", e))?;

    println!("Running binary...\n");
    let status = Command::new(&temp_exe_path).status().map_err(|e| format!("E39 Failed to run binary: {:?}", e))?;

    println!("\nExitted with code: {:?}", status.code());

    println!("Removing temporary file");
    fs::remove_file(temp_exe_path).map_err(|e| format!("E42 Temp file deletion error: {:?}", e))?;

    Ok(())
}