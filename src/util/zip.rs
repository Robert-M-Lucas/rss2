use std::fs;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

pub fn zip_dir_to_bytes<P: AsRef<Path>>(src_dir: P) -> Result<Vec<u8>, String> {
    let mut buffer = Cursor::new(Vec::new());
    let options: FileOptions<()> =
        FileOptions::default().compression_method(CompressionMethod::Stored);

    let base_path = src_dir.as_ref();
    let mut zip_writer = ZipWriter::new(&mut buffer);

    for entry in WalkDir::new(&src_dir) {
        let entry = entry.map_err(|e| format!("{}", e))?;
        let path = entry.path();
        let name = path.strip_prefix(base_path).unwrap().to_str().unwrap();

        if path.is_file() {
            let mut f = File::open(path).map_err(|e| format!("E22 Failed to open file: {}", e))?;
            let mut buffer_file = Vec::new();
            f.read_to_end(&mut buffer_file)
                .map_err(|e| format!("E23 Failed to read file: {}", e))?;
            zip_writer
                .start_file(name, options)
                .map_err(|e| format!("E24 Failed to start zip: {}", e))?;
            zip_writer
                .write_all(&buffer_file)
                .map_err(|e| format!("E25 Failed to write to zip: {}", e))?;
        } else if !name.is_empty() {
            zip_writer
                .add_directory(name.to_string() + "/", options)
                .map_err(|e| format!("E27 Failed to add directory: {}", e))?;
        }
    }

    zip_writer
        .finish()
        .map_err(|e| format!("E26 Failed to finish zip: {}", e))?;
    Ok(buffer.into_inner())
}

pub fn unzip_from_bytes<P: AsRef<Path>>(bytes: &[u8], target_dir: P) -> Result<(), String> {
    let reader = Cursor::new(bytes);
    let mut archive =
        ZipArchive::new(reader).map_err(|e| format!("E28 Failed to open zip: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("E29 Failed to open archive: {}", e))?;
        let outpath = target_dir.as_ref().join(file.mangled_name());

        if file.name().ends_with('/') {
            // Create directory
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("E30 Failed to create directory: {}", e))?;
        } else {
            // Create parent directories if needed
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)
                        .map_err(|e| format!("E31 Failed to create directory: {}", e))?;
                }
            }

            let mut outfile =
                File::create(&outpath).map_err(|e| format!("E32 Failed to create file: {}", e))?;
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("E33 Failed to copy file: {}", e))?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))
                    .map_err(|e| format!("E34 Failed to set permissions of file: {}", e))?;
            }
        }
    }

    Ok(())
}
