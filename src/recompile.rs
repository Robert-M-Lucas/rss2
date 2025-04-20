use crate::config::Config;
use crate::target_triple;
use crate::util::edit_recompile_shared::{
    create_temp_project_dir, extract_project, project_edit_loop,
};
use crate::util::file_contents::FileContents;
use color_print::{cprint, cprintln};
use std::path::Path;
use std::time::Instant;

pub fn recompile<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    let (temp_dir, temp_dir_string, file_name) = create_temp_project_dir(&path)?;

    let mut path_contents = FileContents::from_path(&path)?
        .ok_or(format!("E45 File contents not found: {:?}", path.as_ref()))?;

    extract_project(&path_contents, &temp_dir)?;

    let binary = project_edit_loop(true, config, &temp_dir, &temp_dir_string, &file_name)?;

    let Some(binary) = binary else {
        cprintln!(
            "<red, bold>Failed to compile binary. Use `rss strip [file]` to remove the existing binary.</>"
        );
        return Ok(());
    };

    cprint!("Writing binary ({}) to rss file... ", target_triple());
    path_contents.replace_binary(target_triple(), &binary);
    let start = Instant::now();
    path_contents.save(path)?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);
    path_contents.print_stats();

    Ok(())
}
