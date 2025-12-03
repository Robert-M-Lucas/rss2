use crate::shared::config::Config;
use crate::shared::util::auto_append_rss;
use crate::shared::util::executable::make_executable;
use crate::shared::util::file_contents::FileContents;
use crate::shared::{TARGET_TRIPLE, VERBOSE};
use crate::time;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
#[cfg(windows)]
use std::thread;
#[cfg(windows)]
use std::time::Duration;
use tempfile::NamedTempFile;

pub enum RunParam<P: AsRef<Path>> {
    Path(P),
    Binary(Vec<u8>),
}

pub fn run<P: AsRef<Path>>(
    config: &Config,
    run_param: RunParam<P>,
    args: &[String],
) -> Result<Result<i32, String>, String> {
    let mut _maybe_path_contents = None;
    let bin = match &run_param {
        RunParam::Path(path) => {
            let path = if path.as_ref().is_file() {
                PathBuf::from(path.as_ref())
            } else {
                auto_append_rss(path, config)
            };

            let path_contents = FileContents::from_path(path.as_path())?
                .ok_or(format!("E36 File contents not found: {:?}", path.as_path()))?;

            if path_contents.bin_contents().is_empty() {
                return Ok(Err("rss file has no binary".to_owned()));
            }

            if path_contents.target_triple() != TARGET_TRIPLE {
                return Ok(Err(format!(
                    "File compiled for target '{}', whereas current target is '{}'",
                    path_contents.target_triple(),
                    TARGET_TRIPLE
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
        .args(args)
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

    // Windows can hold a lock on the exe after it has finished executing for a moment
    #[cfg(windows)]
    thread::sleep(Duration::from_millis(1000));

    time!(
        "Removing temporary file",
        false,
        fs::remove_file(temp_exe_path).map_err(|e| format!("E42 Temp file deletion error: {:?}", e))?;
    );

    Ok(Ok(code))
}
