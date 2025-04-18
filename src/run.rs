use crate::config::Config;
use crate::util::file_contents::FileContents;
use color_print::cprintln;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use tempfile::NamedTempFile;
use crate::target_triple;

pub fn run<P: AsRef<Path>>(_config: &Config, path: P) -> Result<(), String> {
    print!("Reading binary from file... ");
    let start = Instant::now();
    let path_contents = FileContents::from_path(&path)?.ok_or(format!("E36 File contents not found: {:?}", path.as_ref()))?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);

    if path_contents.target_triple() != target_triple() {
        return Err(format!("E47 File compiled for target '{}', whereas current target is '{}'", path_contents.target_triple(), target_triple()));
    }

    let temp_exe = NamedTempFile::new().map_err(|e| format!("E37 Temp file creation error: {:?}", e))?;

    if path_contents.bin_contents().len() == 0 {
        return Err("E43 Build failed at last edit resulting in no binary".to_owned());
    }

    print!("Writing binary to temporary file... ");
    let start = Instant::now();
    fs::write(temp_exe.path(), path_contents.bin_contents()).map_err(|e| format!("E38 Temp file creation error: {:?}", e))?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);

    #[cfg(unix)]
    {
        print!("Making temporary file executable (chmod)... ");
        let start = Instant::now();
        Command::new("chmod").args([OsStr::new("+x"), temp_exe.path().as_os_str()]).status().map_err(|e| format!("E40 Failed to run chmod: {:?}", e))?;
        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
    }

    let temp_exe_path = temp_exe.path().to_owned();
    temp_exe.keep().map_err(|e| format!("E41 Failed to mark binary as non-temporary {:?}", e))?;

    println!("Running binary...\n");
    let status = Command::new(&temp_exe_path).status().map_err(|e| format!("E39 Failed to run binary: {:?}", e))?;

    if let Some(code) = status.code() {
        println!("\nExited with code {code}");
    }
    else {
        println!("\nExited with no exit code");
    }

    print!("Removing temporary file... ");
    let start = Instant::now();
    fs::remove_file(temp_exe_path).map_err(|e| format!("E42 Temp file deletion error: {:?}", e))?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);

    Ok(())
}