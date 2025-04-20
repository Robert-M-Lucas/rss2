#[cfg(unix)]
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::fs;
use std::fs::Permissions;
use color_print::cprintln;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

pub fn make_executable<P: AsRef<Path>>(file: P) -> Result<(), String> {
    #[cfg(unix)]
    {
        print!("Making file executable... ");
        let start = Instant::now();
        let current_mode = fs::metadata(file.as_ref()).map_err(|e| format!("E59 Failed to get file permission ({e})"))?.permissions();
        fs::set_permissions(file, Permissions::from_mode(current_mode.mode() | 0o111)).map_err(|e| format!("E60 Failed to set file permissions ({e})"))?;

        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
    }
    Ok(())
}
