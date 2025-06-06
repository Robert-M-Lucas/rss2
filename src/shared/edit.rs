use crate::shared::TARGET_TRIPLE;
use crate::shared::config::Config;
use crate::shared::util::auto_append_rss;
use crate::shared::util::edit_recompile_shared::{
    create_temp_project_dir, extract_project, project_edit_loop,
};
use crate::shared::util::executable::make_executable;
use crate::shared::util::file_contents::FileContents;
use crate::shared::util::zip::zip_dir_to_bytes;
use crate::time;
use color_print::{cformat, cprintln};
use std::path::{Path, PathBuf};
use std::{env, fs};

pub fn edit<P: AsRef<Path>>(config: &Config, path: P, new: bool) -> Result<(), String> {
    let creating = !path.as_ref().is_file();
    if !creating && new {
        return Err("Rss file already exists".to_string());
    }

    let path = if creating {
        auto_append_rss(path, config)
    } else {
        PathBuf::from(path.as_ref())
    };

    let path_contents = FileContents::from_path(&path)?;

    let (temp_dir, temp_dir_string, file_name) = create_temp_project_dir(&path)?;

    let cargo_path = temp_dir.path().join("Cargo.toml");

    if let Some(path_contents) = path_contents {
        extract_project(&path_contents, &temp_dir)?;
    } else {
        time!(
            "Creating default project",
            true,
                fs::write(
            &cargo_path,
            format!(
                "[package]\n\
                name = \"{file_name}\"\n\
                version = \"0.1.0\"\n\
                edition = \"2024\"\n\
                \n\
                [dependencies]\n"
            ),
        )
        .map_err(|e| format!("E09 Failed to create file: {}", e))?;
        fs::create_dir(temp_dir.path().join("src"))
            .map_err(|e| format!("E10 Failed to create directory: {}", e))?;
        fs::write(
            temp_dir.path().join("src").join("main.rs"),
            include_str!("static/main.txt"),
        )
        .map_err(|e| format!("E11 Failed to create file: {}", e))?;
        );
    }

    let cwd = env::current_dir()
        .map_err(|e| format!("E51: Failed to get current working directory: {}", e))?;
    let cr_origin;
    let delete_cr_origin;
    #[cfg(unix)]
    {
        cr_origin = temp_dir.path().join("cr-origin.sh");
        delete_cr_origin = if !cr_origin.is_file() {
            let escaped_path = cwd.to_string_lossy().replace('\'', "'\\''");
            let escaped_temp_cargo_path = cargo_path.to_string_lossy().replace('\'', "'\\''");

            let bash_script = if config.use_debug_mode() {
                format!(
                    "#!/bin/sh\n cd '{}'\ncargo run --manifest-path='{}' \"$@\"",
                    escaped_path, escaped_temp_cargo_path
                )
            } else {
                format!(
                    "#!/bin/sh\ncd '{}'\ncargo run -r --manifest-path='{}' \"$@\"",
                    escaped_path, escaped_temp_cargo_path
                )
            };

            time!(
                cformat!("Creating cr-orig.sh (<yellow, bold>this file will be deleted when saving!</>)"),
                false,
                fs::write(&cr_origin, &bash_script)
                    .map_err(|e| format!("E50 Failed to create cr-origin script: {}", e))?;
            );

            make_executable(&cr_origin)?;
            true
        } else {
            cprintln!("<yellow, bold>Not creating cr-orig.sh as it already exists!</>");
            false
        };
    }
    #[cfg(windows)]
    {
        cr_origin = temp_dir.path().join("cr-origin.cmd");
        delete_cr_origin = if !cr_origin.is_file() {
            let escaped_path = cwd.to_string_lossy().replace('"', "\"\"");
            let escaped_temp_cargo_path = cargo_path.to_string_lossy().replace('"', "\"\"");

            let bash_script = if config.use_debug_mode() {
                format!(
                    "@echo off\r\ncd /d \"{}\"\r\ncargo run --manifest-path=\"{}\" %*",
                    escaped_path, escaped_temp_cargo_path
                )
            } else {
                format!(
                    "@echo off\r\ncd /d \"{}\"\r\ncargo run -r --manifest-path=\"{}\" %*",
                    escaped_path, escaped_temp_cargo_path
                )
            };

            time!(
                cformat!("Creating cr-orig.cmd (<yellow, bold>this file will be deleted when saving!</>)"),
                false,
                fs::write(&cr_origin, &bash_script)
                    .map_err(|e| format!("E54 Failed to create cr-origin script: {}", e))?;
            );

            make_executable(&cr_origin)?;
            true
        } else {
            cprintln!("<yellow, bold>Not creating cr-orig.cmd as it already exists!</>");
            false
        };
    }

    let binary = project_edit_loop(
        false,
        !config.never_save_binary(),
        config,
        &temp_dir,
        &temp_dir_string,
        &file_name,
    )?;

    let target_dir = temp_dir.path().join("target");
    if target_dir.exists() {
        time!(
            "Cleaning up target directory",
            false,
            fs::remove_dir_all(target_dir)
            .map_err(|e| format!("E35 Failed to remove target directory: {}", e))?;
        );
    }

    if delete_cr_origin {
        time!(
            "Deleting cr-origin",
            false,
            fs::remove_file(&cr_origin).map_err(|e| format!("E51 Failed to delete file: {}", e))?;
        );
    }

    let project_zip = time!("Zipping project", false, zip_dir_to_bytes(temp_dir)?);

    let write_description = if binary.is_some() {
        cformat!(
            "Writing rss file <green, bold>(project and binary - {})</>... ",
            TARGET_TRIPLE
        )
    } else {
        cformat!("Writing rss file <red, bold>(no binary)</>... ")
    };

    let file_contents = FileContents::new(project_zip, binary.unwrap_or(vec![]), TARGET_TRIPLE);

    time!(
        write_description,
        false,
        file_contents.save(&path, config)?;
    );

    file_contents.print_stats(
        &path
            .as_path()
            .file_name()
            .ok_or("E63 Failed to read filename from path")?
            .to_string_lossy(),
    );

    Ok(())
}
