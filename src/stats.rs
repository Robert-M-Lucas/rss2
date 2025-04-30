use crate::config::Config;
use crate::util::file_contents::FileContents;
use std::path::Path;

pub fn stats<P: AsRef<Path>>(_config: &Config, path: P) -> Result<(), String> {
    let path_contents = FileContents::from_path(&path)?.ok_or(format!(
        "E78 File contents not found: '{}'",
        path.as_ref().to_string_lossy()
    ))?;

    let file_name = path
        .as_ref()
        .file_name()
        .ok_or("E79 Failed to directory name")?
        .to_string_lossy();

    path_contents.print_stats(&file_name);

    Ok(())
}
