use crate::shared::VERBOSE;
use crate::shared::config::get_config;
use crate::shared::recompile::recompile;
use crate::shared::run::{RunParam, run};
use color_print::cprintln;
use std::process::exit;

pub fn wrapped_run(file: &str, args: &[String]) -> Result<(), String> {
    let config = get_config()?;
    let binary_exists = run(&config, RunParam::Path(&file), args)?;

    // Build and re-run if binary doesn't exist
    let code = match binary_exists {
        Ok(code) => code,
        Err(no_binary_reason) => {
            cprintln!("<yellow, bold>[!] {no_binary_reason} - recompiling...</>");
            let compiled_binary = recompile(&config, file)?;
            if let Some(compiled_binary) = compiled_binary {
                if !VERBOSE.get().unwrap() {
                    println!("Running binary...");
                }
                run(&config, RunParam::<String>::Binary(compiled_binary), args)?.unwrap_or(-1)
            } else {
                -1
            }
        }
    };

    exit(code);
}
