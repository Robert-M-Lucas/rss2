use crate::shared::VERBOSE;
use crate::shared::config::Config;
use color_print::cprintln;
use std::path::{Path, PathBuf};

pub mod edit_recompile_shared;
pub mod executable;
pub mod file_contents;
pub mod zip;

pub fn auto_append_rss<P: AsRef<Path>>(path: P, config: &Config) -> PathBuf {
    if config.auto_append_rss_ext() && path.as_ref().extension().is_none_or(|e| e != "rss") {
        let with_rss = PathBuf::from(path.as_ref()).with_extension("rss");
        if *VERBOSE.get().unwrap() {
            cprintln!(
                "<yellow,bold>Using {:?} instead of {:?} (auto_append_rss_ext is set to true in config)</>",
                with_rss,
                path.as_ref()
            );
        }
        with_rss
    } else {
        path.as_ref().to_path_buf()
    }
}

#[macro_export]
macro_rules! time {
    ($description:expr, $important:expr, $($tts:tt)*) => {
        {
            if ($important) || (*$crate::shared::VERBOSE.get().unwrap()) {
                $crate::print_task_start!($description);
            }
            let start = std::time::Instant::now();
            let val = {$($tts)*};
            let time = start.elapsed();
            if (*$crate::shared::VERBOSE.get().unwrap()) {
                $crate::println_task_duration!(time);
            }
            else if $important {
                println!();
            }
            val
        }
    };
}

#[macro_export]
macro_rules! time_leak_scope {
    ($description:expr, $important:expr, $($tts:tt)*) => {
        if ($important) || (*$crate::shared:::VERBOSE.get().unwrap()) {
            $crate::print_task_start!($description);
        }
        let start = std::time::Instant::now();
        $($tts)*;
        let time = start.elapsed();
        if (*$crate::shared:::VERBOSE.get().unwrap()) {
                $crate::println_task_duration!(time);
            }
            else if $important {
                println!();
            }
    };
}

#[macro_export]
macro_rules! print_task_start {
    ($description:expr) => {
        print!("{}... ", $description);
    };
}

#[macro_export]
macro_rules! println_task_duration {
    ($end:expr) => {
        color_print::cprintln!("<cyan,bold>[{:?}]</>", $end)
    };
}
