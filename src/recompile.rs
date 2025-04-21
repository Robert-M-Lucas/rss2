use crate::config::Config;
use crate::util::edit_recompile_shared::{
    create_temp_project_dir, extract_project, project_edit_loop,
};
use crate::util::file_contents::FileContents;
use crate::{target_triple, time};
use color_print::{cformat, cprintln};
use std::path::Path;

pub fn recompile<P: AsRef<Path>>(config: &Config, path: P) -> Result<Option<Vec<u8>>, String> {
    let (temp_dir, temp_dir_string, file_name) = create_temp_project_dir(&path)?;

    let mut path_contents = FileContents::from_path(&path)?
        .ok_or(format!("E45 File contents not found: {:?}", path.as_ref()))?;

    extract_project(&path_contents, &temp_dir)?;

    let binary = project_edit_loop(true, true, config, &temp_dir, &temp_dir_string, &file_name)?;

    let Some(binary) = binary else {
        cprintln!(
            "<red, bold>Failed to compile binary. Use `rss strip [file]` to remove the existing binary.</>"
        );
        return Ok(None);
    };

    if *config.never_save_binary() {
        cprintln!("<yellow, bold>Not saving compiled binary due to config</>");
        return Ok(Some(binary));
    }

    path_contents.replace_binary(target_triple(), &binary);
    time!(
        cformat!("Writing binary ({}) to rss file... ", target_triple()),
        path_contents.save(&path)?;
    );

    path_contents.print_stats(
        &path
            .as_ref()
            .file_name()
            .ok_or("E62 Failed to read filename from path")?
            .to_string_lossy(),
    );

    Ok(Some(binary))
}
