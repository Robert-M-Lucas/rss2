#[cfg(unix)]
use crate::time;
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::fs::Permissions;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use std::path::Path;

pub fn make_executable<P: AsRef<Path>>(file: P) -> Result<(), String> {
    #[cfg(unix)]
    {
        time!(
            "Making file executable... ",
            let current_mode = fs::metadata(file.as_ref())
                .map_err(|e| format!("E59 Failed to get file permission ({e})"))?
                .permissions();
            fs::set_permissions(file, Permissions::from_mode(current_mode.mode() | 0o111))
                .map_err(|e| format!("E60 Failed to set file permissions ({e})"))?;
        );
    }
    #[cfg(windows)]
    let _ = file;
    Ok(())
}
