use crate::config::Config;
use crate::util::file_contents::FileContents;
use color_print::cprintln;
use std::path::Path;
use std::time::Instant;

pub fn strip<P: AsRef<Path>>(_config: &Config, path: P) -> Result<(), String> {
    print!("Reading file... ");
    let start = Instant::now();
    let mut path_contents = FileContents::from_path(&path)?
        .ok_or(format!("E44 File contents not found: {:?}", path.as_ref()))?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);

    path_contents.remove_binary();

    print!("Saving stripped file... ");
    let start = Instant::now();
    path_contents.save(path)?;
    let time = Instant::now() - start;
    cprintln!("<cyan>[{:?}]</>", time);
    path_contents.print_stats();
    Ok(())
}
