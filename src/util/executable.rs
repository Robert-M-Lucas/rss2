#[cfg(unix)]
use color_print::cprintln;
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::fs::Permissions;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
#[cfg(unix)]
use std::time::Instant;
use std::path::Path;

pub fn make_executable<P: AsRef<Path>>(file: P) -> Result<(), String> {
    #[cfg(unix)]
    {
        print!("Making file executable... ");
        let start = Instant::now();
        let current_mode = fs::metadata(file.as_ref())
            .map_err(|e| format!("E59 Failed to get file permission ({e})"))?
            .permissions();
        fs::set_permissions(file, Permissions::from_mode(current_mode.mode() | 0o111))
            .map_err(|e| format!("E60 Failed to set file permissions ({e})"))?;

        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
    }
    #[cfg(windows)]
    let _ = file;
    Ok(())
}
