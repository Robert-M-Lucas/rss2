use crate::config::Config;
use crate::util::file_contents::FileContents;
use crate::util::zip::print_tree;
use std::path::Path;

pub fn tree<P: AsRef<Path>>(_config: &Config, path: P) -> Result<(), String> {
    let file_name = path
        .as_ref()
        .file_name()
        .ok_or("E80 Failed to directory name")?
        .to_string_lossy();

    let path_contents = FileContents::from_path(&path)?.ok_or(format!(
        "E81 File contents not found: '{}'",
        path.as_ref().to_string_lossy()
    ))?;

    print_tree(path_contents.zipped_contents(), &file_name)?;

    Ok(())
}
