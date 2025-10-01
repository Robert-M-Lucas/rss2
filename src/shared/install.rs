use std::path::{Path, PathBuf};
use crate::shared::config::Config;
use crate::shared::util::auto_append_rss;
use crate::shared::util::edit_recompile_shared::{create_temp_project_dir, extract_project, project_edit_loop, EditLoopMode};
use crate::shared::util::file_contents::FileContents;

pub fn install<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    let path = if path.as_ref().is_file() {
        PathBuf::from(path.as_ref())
    } else {
        auto_append_rss(path, config)
    };

    let path_contents = FileContents::from_path(&path)?
        .ok_or(format!("E87 File contents not found: {:?}", path.as_path()))?;

    let (temp_dir, temp_dir_string, file_name) = create_temp_project_dir(&path)?;

    extract_project(&path_contents, &temp_dir)?;

    project_edit_loop(true, EditLoopMode::Install, config, &temp_dir, &temp_dir_string, &file_name)?;
    
    Ok(())
}