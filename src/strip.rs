use crate::config::Config;
use crate::time;
use crate::util::file_contents::FileContents;
use std::path::Path;

pub fn strip<P: AsRef<Path>>(_config: &Config, path: P) -> Result<(), String> {
    let mut path_contents = time!(
        "Reading file... ",
        FileContents::from_path(&path)?
            .ok_or(format!("E44 File contents not found: {:?}", path.as_ref()))?
    );

    path_contents.remove_binary();

    time!(
        "Saving stripped file... ",
        path_contents.save(&path)?;
    );

    path_contents.print_stats(
        &path
            .as_ref()
            .file_name()
            .ok_or("E61 Failed to read filename from path")?
            .to_string_lossy(),
    );
    Ok(())
}
