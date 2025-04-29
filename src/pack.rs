use crate::config::Config;
use crate::util::edit_recompile_shared::{
    create_temp_project_dir, extract_project, project_edit_loop,
};
use crate::util::file_contents::FileContents;
use crate::util::zip::zip_dir_to_bytes;
use crate::{target_triple, time};
use color_print::{cformat, cprintln};
use std::fs;
use std::path::Path;

pub fn pack<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    let dir = path
        .as_ref()
        .canonicalize()
        .map_err(|e| format!("E70 Error parsing path: {:?}", e))?;

    let dir_string = dir.to_str().ok_or("E71 Failed to get directory path")?;

    let dir_name = dir
        .file_name()
        .ok_or("E72 Failed to directory name")?
        .to_string_lossy();

    let binary = project_edit_loop(
        true,
        !*config.never_save_binary(),
        config,
        &dir,
        dir_string,
        &dir_name,
    )?;

    let target_dir = dir.as_path().join("target");
    if target_dir.exists() {
        time!(
            "Cleaning up target directory",
            false,
            fs::remove_dir_all(target_dir)
            .map_err(|e| format!("E73 Failed to remove target directory: {}", e))?;
        );
    }

    let project_zip = time!("Zipping project", false, zip_dir_to_bytes(&dir)?);

    let write_description = if binary.is_some() {
        cformat!(
            "Writing rss file <green, bold>(project and binary - {})</>... ",
            target_triple()
        )
    } else {
        cformat!("Writing rss file <red, bold>(no binary)</>... ")
    };

    let file_contents = FileContents::new(project_zip, binary.unwrap_or(vec![]), target_triple());

    let file_name = format!("{dir_name}.rss");
    time!(
        write_description,
        false,
        file_contents.save(&file_name)?;
    );

    file_contents.print_stats(&file_name);

    Ok(())
}
