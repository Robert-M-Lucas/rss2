#[cfg(unix)]
use std::ffi::OsStr;

use color_print::cprintln;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

pub fn make_executable<P: AsRef<Path>>(file: P) -> Result<(), String> {
    #[cfg(unix)]
    {
        print!("Making file executable (chmod)... ");
        let start = Instant::now();
        // TODO: Might be possible without running chmod (directly modifying unix file flags)
        let status = Command::new("chmod")
            .args([OsStr::new("+x"), file.as_ref().as_os_str()])
            .status()
            .map_err(|e| format!("E40 Failed to run chmod: {:?}", e))?;

        if !status.success() {
            return Err(format!("E52 Failed to run chmod: {}", status.to_string()));
        }

        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
    }
    Ok(())
}
