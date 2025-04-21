use crate::config::Config;
use crate::util::executable::make_executable;
use crate::util::file_contents::FileContents;
use crate::{VERBOSE, target_triple, time};
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

pub enum RunParam<P: AsRef<Path>> {
    Path(P),
    Binary(Vec<u8>),
}

pub fn run<P: AsRef<Path>>(
    _config: &Config,
    run_param: RunParam<P>,
) -> Result<Result<i32, String>, String> {
    let mut _maybe_path_contents = None;
    let bin = match &run_param {
        RunParam::Path(path) => {
            let path_contents = FileContents::from_path(path)?
                .ok_or(format!("E36 File contents not found: {:?}", path.as_ref()))?;

            if path_contents.bin_contents().is_empty() {
                return Ok(Err("rss file has no binary".to_owned()));
            }

            if path_contents.target_triple() != target_triple() {
                return Ok(Err(format!(
                    "File compiled for target '{}', whereas current target is '{}'",
                    path_contents.target_triple(),
                    target_triple()
                )));
            }
            _maybe_path_contents = Some(path_contents);
            _maybe_path_contents.as_ref().unwrap().bin_contents()
        }
        RunParam::Binary(b) => b,
    };

    let temp_exe =
        NamedTempFile::new().map_err(|e| format!("E37 Temp file creation error: {:?}", e))?;

    time!(
        "Writing binary to temporary file",
        false,
        fs::write(temp_exe.path(), bin)
        .map_err(|e| format!("E38 Temp file creation error: {:?}", e))?;
    );

    make_executable(&temp_exe)?;

    let temp_exe_path = temp_exe.path().to_owned();
    temp_exe
        .keep()
        .map_err(|e| format!("E41 Failed to mark binary as non-temporary {:?}", e))?;

    if *VERBOSE.get().unwrap() {
        println!("Running binary...");
    }

    let status = Command::new(&temp_exe_path)
        .status()
        .map_err(|e| format!("E39 Failed to run binary: {:?}", e))?;

    let code = if let Some(code) = status.code() {
        if *VERBOSE.get().unwrap() {
            println!("\nExited with code {code}");
        }
        code
    } else {
        if *VERBOSE.get().unwrap() {
            println!("\nExited with no exit code");
        }
        0
    };

    time!(
        "Removing temporary file",
        false,
        fs::remove_file(temp_exe_path).map_err(|e| format!("E42 Temp file deletion error: {:?}", e))?;
    );

    Ok(Ok(code))
}
