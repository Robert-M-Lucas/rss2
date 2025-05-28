use crate::shared::config::Config;
use crate::shared::util::auto_append_rss;
use crate::shared::util::file_contents::FileContents;
use crate::time;
use std::path::{Path, PathBuf};

pub fn strip<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    let path = if path.as_ref().is_file() {
        PathBuf::from(path.as_ref())
    } else {
        auto_append_rss(path, config)
    };

    let mut path_contents = time!(
        "Reading file",
        false,
        FileContents::from_path(&path)?
            .ok_or(format!("E44 File contents not found: {:?}", path.as_path()))?
    );

    path_contents.remove_binary();

    time!(
        "Saving stripped file",
        false,
        path_contents.save(&path, config)?;
    );

    path_contents.print_stats(
        &path
            .as_path()
            .file_name()
            .ok_or("E61 Failed to read filename from path")?
            .to_string_lossy(),
    );
    Ok(())
}
