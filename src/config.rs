pub mod edit_command;

use color_print::{cprint, cprintln};
use derive_getters::Getters;
use directories::BaseDirs;
use edit_command::EditCommand;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, Default, Getters)]
#[serde(default)]
pub struct Config {
    config_edit_command: EditCommand,
    rust_project_edit_command: EditCommand,
    use_debug_mode: bool,
}

impl Config {
    #[allow(dead_code)]
    pub fn save(&self) -> Result<(), String> {
        let config_file = get_config_path()?;
        let json = serde_json::to_string_pretty(&self).map_err(|_| "E13 Failed to serialise config".to_owned())?;
        fs::write(&config_file, json).map_err(|_| "E14 Failed to write config file".to_owned())?;
        Ok(())
    }
}

pub fn get_config_path() -> Result<PathBuf, String> {
    let Some(config_dir) = BaseDirs::new().and_then(|bd| Some(bd.config_dir().to_owned())) else {
        return Err("E03 Failed to get config directory".to_owned());
    };

    Ok(config_dir.join("rss-config.json"))
}

pub fn get_config() -> Result<Config, String> {
    let mut cancel_time = false;
    print!("Fetching config... ");
    let start = Instant::now();

    let config_file = get_config_path()?;

    let r = Ok(if Path::new(&config_file).exists() {
        let mut file = File::open(&config_file).map_err(|_| "E15 Failed to open config file".to_owned())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|_| "E16 Failed to read config file".to_owned())?;
        serde_json::from_str(&contents).map_err(|_| "E17 Failed to parse config file".to_owned())?
    } else {
        cancel_time = true;
        println!("\nConfig file not found. Creating default at '{}'.", get_config_path()?.as_os_str().to_string_lossy());
        cprintln!("<yellow, bold>IMPORTANT: Change the editor in the config if you do not have VS Code in path!</>");
        print!("Press enter to continue...");
        stdout().flush().ok();
        let mut t = String::new();
        stdin().read_line(&mut t).ok();
        let config = Config::default();
        let json = serde_json::to_string_pretty(&config).map_err(|_| "E18 Failed to serialize config".to_owned())?;
        print!("Writing config... ");
        let start = Instant::now();
        fs::write(&config_file, json).map_err(|_| "E19 Failed to write config file".to_owned())?;
        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
        config
    });
    if !cancel_time {
        let time = Instant::now() - start;
        cprintln!("<cyan>[{:?}]</>", time);
    }
    r
}

pub fn reset_config() -> Result<(String, String), String> {
    let config_file = get_config_path()?;
    let config = Config::default();
    let json = serde_json::to_string_pretty(&config).map_err(|_| "E20 Failed to serialize config".to_owned())?;
    fs::write(&config_file, &json).map_err(|_| "E21 Failed to write config file".to_owned())?;
    Ok((config_file.as_os_str().to_string_lossy().to_string(), json))
}

pub fn edit_config(config: &Config) -> Result<(), String> {
    let config_path = get_config_path()?;
    let Some(config_path) = config_path.to_str().to_owned() else {
        return Err("E02 Failed to get config path".to_owned());
    };
    if let Err(e) = config.config_edit_command().to_command(Some(&config_path))?.output() {
        return Err(format!("E48 Error when running config edit command: {}\n\
        Check/edit the command used in '{}'.", e, get_config_path()?.as_os_str().to_string_lossy()));
    }
    Ok(())
}