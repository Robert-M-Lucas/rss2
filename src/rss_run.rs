#![allow(dead_code)]
#[macro_use]
extern crate const_it;
#[macro_use]
extern crate static_assertions;

use crate::shared::wrapped_run::wrapped_run;
use color_print::cprintln;

mod shared;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() >= 2 {
        if let Err(e) = wrapped_run(&args[1], &args[2..]) {
            cprintln!("\n<red, bold>{e}</>");
        }
    } else {
        cprintln!("<red, bold>File not provided. rss-run should not be used manually.</>")
    }
}
