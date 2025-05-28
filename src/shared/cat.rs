use crate::shared::config::Config;
use crate::shared::util::auto_append_rss;
use crate::shared::util::file_contents::FileContents;
use crate::shared::util::zip::{Filter, cat_files};
use color_print::cprintln;
use std::path::{Path, PathBuf};

pub fn cat<P: AsRef<Path>>(
    config: &Config,
    path: P,
    name: Option<&str>,
    extension: Option<&str>,
    all: bool,
) -> Result<(), String> {
    let path = if path.as_ref().is_file() {
        PathBuf::from(path.as_ref())
    } else {
        auto_append_rss(path, config)
    };

    if name.is_some() && extension.is_some() {
        return Err("E83 Both `name` and `extension` flags cannot be used together.".to_string());
    }

    if (extension.is_some() || name.is_some()) && all {
        cprintln!(
            "<yellow, bold>Using the `all` flag is redundant when using the `extension` or `name` flag.</>"
        );
    }

    let filter = if let Some(name) = name {
        Filter::Name(name.to_string())
    } else if let Some(extension) = extension {
        Filter::Extension(extension.to_string())
    } else if all {
        Filter::None
    } else {
        Filter::Extension("rs".to_string())
    };

    let path_contents = FileContents::from_path(&path)?
        .ok_or(format!("E81 File contents not found: {:?}", path.as_path()))?;

    cat_files(path_contents.zipped_contents(), filter)?;

    Ok(())
}
