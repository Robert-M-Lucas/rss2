use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Getters)]
#[serde(default)]
pub struct EditCommand {
    command: String,
    args: Vec<String>,
}

impl EditCommand {
    pub fn to_command(&self, insert: Option<&str>) -> Result<Command, String> {
        let mut o_command = Command::new(&self.command);
        let mut command = &mut o_command;

        let mut inserted = false;
        for arg in &self.args {
            if arg == "$$$" {
                if let Some(insert) = insert {
                    command.arg(insert);
                    inserted = true;
                } else {
                    return Err("E01 Found $$$ in command with no insertion".to_owned());
                }
            } else {
                command = command.arg(arg);
            }
        }

        if insert.is_some() && !inserted {
            return Err("E04 One argument must be '$$$' for argument insertion".to_owned());
        }
        Ok(o_command)
    }
}

impl Default for EditCommand {
    fn default() -> Self {
        EditCommand {
            command: "code".to_owned(),
            args: vec!["-w".to_owned(), "$$$".to_owned()],
        }
    }
}
