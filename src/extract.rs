use crate::config::Config;
use crate::util::edit_recompile_shared::extract_project;
use crate::util::file_contents::FileContents;
use color_print::cprintln;
use std::fs;
use std::path::Path;
use std::time::Instant;

pub fn extract<P: AsRef<Path>>(_config: &Config, path: P) -> Result<(), String> {
    let path_contents = FileContents::from_path(&path)?.ok_or(format!(
        "E64 File contents not found: '{}'",
        path.as_ref().to_string_lossy()
    ))?;

    let dir = path
        .as_ref()
        .parent()
        .ok_or("E66 Error parsing path".to_owned())?
        .join(
            path.as_ref()
                .file_stem()
                .ok_or("E67 Error parsing file name".to_owned())?,
        );

    print!("Creating directory... ");
    let start = Instant::now();
    fs::create_dir(&dir).map_err(|e| {
        format!(
            "E65 Could not create directory '{}' - {e}",
            dir.to_string_lossy()
        )
    })?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);

    extract_project(&path_contents, &dir)?;

    Ok(())
}
