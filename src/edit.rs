use crate::config::Config;
use crate::target_triple;
use crate::util::edit_recompile_shared::{
    create_temp_project_dir, extract_project, project_edit_loop,
};
use crate::util::executable::make_executable;
use crate::util::file_contents::FileContents;
use crate::util::zip::zip_dir_to_bytes;
use color_print::{cprint, cprintln};
use std::path::Path;
use std::time::Instant;
use std::{env, fs};

pub fn edit<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    let (temp_dir, temp_dir_string, file_name) = create_temp_project_dir(&path)?;

    let path_contents = FileContents::from_path(&path)?;
    let cargo_path = temp_dir.path().join("Cargo.toml");

    if let Some(path_contents) = path_contents {
        extract_project(&path_contents, &temp_dir)?;
    } else {
        print!("Creating default project... ");
        let start = Instant::now();
        fs::write(
            &cargo_path,
            format!(
                "[package]
name = \"{file_name}\"
version = \"0.1.0\"
edition = \"2024\"

[dependencies]
"
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
        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
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

            let bash_script = if *config.use_debug_mode() {
                format!(
                    "#!/bin/sh\n cd '{}'\ncargo run --manifest-path='{}'",
                    escaped_path, escaped_temp_cargo_path
                )
            } else {
                format!(
                    "#!/bin/sh\ncd '{}'\ncargo run -r --manifest-path='{}'",
                    escaped_path, escaped_temp_cargo_path
                )
            };

            cprint!(
                "Creating cr-orig.sh (<yellow, bold>this file will be deleted when saving!</>)... "
            );
            let start = Instant::now();
            fs::write(&cr_origin, &bash_script)
                .map_err(|e| format!("E50 Failed to create cr-origin script: {}", e))?;
            let time = Instant::now() - start;
            cprintln!("<cyan>[{:?}]</>", time);
            make_executable(&cr_origin)?;
            true
        } else {
            cprintln!("<yellow>Not creating cr-orig.sh as it already exists!</>");
            false
        };
    }
    #[cfg(windows)]
    {
        cr_origin = temp_dir.path().join("cr-origin.cmd");
        delete_cr_origin = if !cr_origin.is_file() {
            let escaped_path = cwd.to_string_lossy().replace('"', "\"\"");
            let escaped_temp_cargo_path = cargo_path.to_string_lossy().replace('"', "\"\"");

            let bash_script = if *config.use_debug_mode() {
                format!(
                    "cd /d \"{}\"\r\ncargo run --manifest-path=\"{}\"",
                    escaped_path, escaped_temp_cargo_path
                )
            } else {
                format!(
                    "cd /d \"{}\"\r\ncargo run -r --manifest-path=\"{}\"",
                    escaped_path, escaped_temp_cargo_path
                )
            };

            cprint!(
                "Creating cr-orig.cmd (<yellow, bold>this file will be deleted when saving!</>)... "
            );
            let start = Instant::now();
            fs::write(&cr_origin, &bash_script)
                .map_err(|e| format!("E54 Failed to create cr-origin script: {}", e))?;
            let time = Instant::now() - start;
            cprintln!("<cyan>[{:?}]</>", time);
            make_executable(&cr_origin)?;
            true
        } else {
            cprintln!("<yellow>Not creating cr-orig.cmd as it already exists!</>");
            false
        };
    }

    let binary = project_edit_loop(false, config, &temp_dir, &temp_dir_string, &file_name)?;

    print!("Cleaning up target directory... ");
    let start = Instant::now();
    fs::remove_dir_all(temp_dir.path().join("target"))
        .map_err(|e| format!("E35 Failed to remove temporary directory: {}", e))?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);

    if delete_cr_origin {
        print!("Deleting cr-origin... ");
        let start = Instant::now();
        fs::remove_file(&cr_origin).map_err(|e| format!("E51 Failed to delete file: {}", e))?;
        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
    }

    print!("Zipping project... ");
    let start = Instant::now();
    let project_zip = zip_dir_to_bytes(temp_dir)?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);

    if binary.is_some() {
        cprint!(
            "Writing rss file <green, bold>(project and binary - {})</>... ",
            target_triple()
        );
    } else {
        cprint!("Writing rss file <red, bold>(no binary)</>... ");
    }
    let file_contents = FileContents::new(project_zip, binary.unwrap_or(vec![]), target_triple());
    let start = Instant::now();
    file_contents.save(path)?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);
    file_contents.print_stats();

    Ok(())
}
