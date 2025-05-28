use crate::shared::config::Config;
use crate::shared::util::auto_append_rss;
use crate::shared::util::file_contents::FileContents;
use std::path::{Path, PathBuf};

pub fn stats<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    let path = if path.as_ref().is_file() {
        PathBuf::from(path.as_ref())
    } else {
        auto_append_rss(path, config)
    };

    let path_contents = FileContents::from_path(&path)?.ok_or(format!(
        "E78 File contents not found: '{}'",
        path.as_path().to_string_lossy()
    ))?;

    let file_name = path
        .as_path()
        .file_name()
        .ok_or("E79 Failed to directory name")?
        .to_string_lossy();

    path_contents.print_stats(&file_name);

    Ok(())
}
