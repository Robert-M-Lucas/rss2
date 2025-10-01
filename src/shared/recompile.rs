use crate::shared::TARGET_TRIPLE;
use crate::shared::config::Config;
use crate::shared::util::auto_append_rss;
use crate::shared::util::edit_recompile_shared::{create_temp_project_dir, extract_project, project_edit_loop, EditLoopMode};
use crate::shared::util::file_contents::FileContents;
use crate::time;
use color_print::{cformat, cprintln};
use std::path::{Path, PathBuf};

pub fn recompile<P: AsRef<Path>>(config: &Config, path: P) -> Result<Option<Vec<u8>>, String> {
    let path = if path.as_ref().is_file() {
        PathBuf::from(path.as_ref())
    } else {
        auto_append_rss(path, config)
    };

    let mut path_contents = FileContents::from_path(&path)?
        .ok_or(format!("E45 File contents not found: {:?}", path.as_path()))?;
    
    let (temp_dir, temp_dir_string, file_name) = create_temp_project_dir(&path)?;

    extract_project(&path_contents, &temp_dir)?;

    let binary = project_edit_loop(true, EditLoopMode::CompileBinary, config, &temp_dir, &temp_dir_string, &file_name)?;

    let Some(binary) = binary else {
        cprintln!(
            "<red, bold>Failed to compile binary. Use `rss strip [file]` to remove the existing binary.</>"
        );
        return Ok(None);
    };

    if config.never_save_binary() {
        cprintln!("<yellow, bold>Not saving compiled binary due to config</>");
        return Ok(Some(binary));
    }

    path_contents.replace_binary(TARGET_TRIPLE, &binary);
    time!(
        cformat!("Writing binary ({}) to rss file", TARGET_TRIPLE),
        false,
        path_contents.save(&path, config)?;
    );

    path_contents.print_stats(
        &path
            .as_path()
            .file_name()
            .ok_or("E62 Failed to read filename from path")?
            .to_string_lossy(),
    );

    Ok(Some(binary))
}
