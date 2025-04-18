use crate::config::Config;
use crate::util::edit_recompile_shared::{create_temp_project_dir, extract_project, project_edit_loop};
use crate::util::file_contents::FileContents;
use crate::util::zip::zip_dir_to_bytes;
use color_print::{cprint, cprintln};
use std::fs;
use std::path::Path;
use std::time::Instant;
use crate::target_triple;

pub fn edit<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), String> {
    let (temp_dir, temp_dir_string, file_name) = create_temp_project_dir(&path)?;

    let path_contents = FileContents::from_path(&path)?;

    if let Some(path_contents) = path_contents {
        extract_project(&path_contents, &temp_dir)?;
    }
    else {
        print!("Creating default project... ");
        let start = Instant::now();
        fs::write(temp_dir.path().join("Cargo.toml"), format!("[package]
name = \"{file_name}\"
version = \"0.1.0\"
edition = \"2024\"

[dependencies]
")).map_err(|e| format!("E09 Failed to create file: {}", e))?;
        fs::create_dir(temp_dir.path().join("src")).map_err(|e| format!("E10 Failed to create directory: {}", e))?;
        fs::write(temp_dir.path().join("src").join("main.rs"), "fn main() {
    println!(\"Hello, world!\");
}").map_err(|e| format!("E11 Failed to create file: {}", e))?;
        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
    }

    let binary = project_edit_loop(false, config, &temp_dir, &temp_dir_string, &file_name)?;

    print!("Cleaning up target directory... ");
    let start = Instant::now();
    fs::remove_dir_all(temp_dir.path().join("target")).map_err(|e| format!("E35 Failed to remove temporary directory: {}", e))?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);

    print!("Zipping project... ");
    let start = Instant::now();
    let project_zip = zip_dir_to_bytes(temp_dir)?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);


    if binary.is_some() {
        cprint!("Writing rss file <green, bold>(project and binary - {})</>... ", target_triple());
    }
    else {
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