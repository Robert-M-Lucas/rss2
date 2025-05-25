use crate::config::Config;
use crate::util::auto_append_rss;
use crate::util::file_contents::FileContents;
use crate::util::zip::print_tree;
use std::path::{Path, PathBuf};

pub fn tree<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    let path = if path.as_ref().is_file() {
        PathBuf::from(path.as_ref())
    } else {
        auto_append_rss(path, config)
    };

    let file_name = path
        .as_path()
        .file_name()
        .ok_or("E80 Failed to directory name")?
        .to_string_lossy();

    let path_contents = FileContents::from_path(&path)?.ok_or(format!(
        "E81 File contents not found: '{}'",
        path.as_path().to_string_lossy()
    ))?;

    print_tree(path_contents.zipped_contents(), &file_name)?;

    Ok(())
}
