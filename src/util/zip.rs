use color_print::cprintln;
use std::borrow::Cow;
use std::fs;
use std::fs::File;
use std::io::{BufRead, Cursor, Read, Write};
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

#[derive(Debug)]
struct FileTree {
    name: String,
    hidden: bool,
    children: Vec<FileTree>,
}

impl FileTree {
    fn new(name: String, hidden: bool) -> Self {
        Self {
            name,
            hidden,
            children: Vec::new(),
        }
    }

    fn add_path(&mut self, new: &[Cow<str>], hidden: bool) {
        if new.is_empty() {
            return;
        }
        if let Some(tree) = self.children.iter_mut().find(|t| t.name == new[0]) {
            if new.len() == 1 {
                tree.hidden = hidden;
            } else {
                tree.add_path(&new[1..], hidden);
            }
        } else if new.len() == 1 {
            self.children
                .push(FileTree::new(new[0].to_string(), hidden));
        } else {
            self.children.push(FileTree::new(new[0].to_string(), false));
            self.children
                .last_mut()
                .unwrap()
                .add_path(&new[1..], hidden);
        }
    }

    pub fn print(&self, show_hidden: bool) {
        self.print_int(String::new(), true, true, show_hidden);
    }

    fn print_int(&self, prefix: String, is_last: bool, is_top: bool, show_hidden: bool) {
        if is_top {
            println!("{}", self.name)
        } else {
            println!(
                "{}{} {}",
                prefix,
                if is_last { "└──" } else { "├──" },
                self.name
            );
        }

        let new_prefix = if is_top {
            String::new()
        } else {
            format!("{}{}", prefix, if is_last { "    " } else { "│   " })
        };
        let len = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            if !show_hidden && child.hidden {
                continue;
            }
            child.print_int(new_prefix.clone(), i == len - 1, false, show_hidden);
        }
    }
}

pub fn print_tree(bytes: &[u8], file_name: &str) -> Result<(), String> {
    let reader = Cursor::new(bytes);
    let mut archive =
        ZipArchive::new(reader).map_err(|e| format!("E82 Failed to open zip: {}", e))?;

    let mut tree = FileTree::new(file_name.to_string(), false);

    for i in 0..archive.len() {
        let file = archive
            .by_index(i)
            .map_err(|e| format!("E29 Failed to open archive: {}", e))?;

        let mangled = file.mangled_name();
        let sections = mangled
            .iter()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>();

        tree.add_path(&sections, false);
    }

    tree.print(true);

    Ok(())
}

pub enum Filter {
    None,
    Name(String),
    Extension(String),
}

pub fn cat_files(bytes: &[u8], filter: Filter) -> Result<(), String> {
    let reader = Cursor::new(bytes);
    let mut archive =
        ZipArchive::new(reader).map_err(|e| format!("E28 Failed to open zip: {}", e))?;

    let mut shown = false;
    for i in 0..archive.len() {
        let file = archive
            .by_index(i)
            .map_err(|e| format!("E29 Failed to open archive: {}", e))?;
        if file.is_dir() {
            continue;
        }
        let path = file.mangled_name();

        match &filter {
            Filter::None => {}
            Filter::Name(name) => {
                if path
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .is_none_or(|n| &n != name)
                {
                    continue;
                }
            }
            Filter::Extension(extension) => {
                if path
                    .extension()
                    .map(|e| e.to_string_lossy())
                    .is_none_or(|e| &e != extension)
                {
                    continue;
                }
            }
        }

        shown = true;

        println!("{}", path.to_string_lossy());

        let mut lines = std::io::BufReader::new(file).lines();
        while let Some(Ok(line)) = lines.next() {
            cprintln!("<cyan>  │ </>{}", line);
        }

        println!();
    }

    if !shown {
        cprintln!("<yellow, bold>No files found</>")
    }

    Ok(())
}
