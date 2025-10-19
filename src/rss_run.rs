#![allow(dead_code)]
#[macro_use]
extern crate const_it;
#[macro_use]
extern crate static_assertions;

use crate::shared::VERBOSE;
use crate::shared::ctrl_c_handler::init_ctrl_c_handler;
use crate::shared::wrapped_run::wrapped_run;
use color_print::cprintln;

mod shared;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    VERBOSE.set(false).unwrap();

    // Not used for rss-run but may be in future
    init_ctrl_c_handler();

    if args.len() >= 2 {
        if let Err(e) = wrapped_run(&args[1], &args[2..]) {
            cprintln!("\n<red, bold>{e}</>");
        }
    } else {
        cprintln!("<red, bold>File not provided. rss-run should not be used manually.</>")
    }
}
