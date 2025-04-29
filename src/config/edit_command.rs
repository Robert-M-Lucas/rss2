use derive_getters::Getters;
use either::{Either, Right};
use serde::{Deserialize, Serialize};
use std::process::{Command, ExitStatus, Output};

#[derive(Debug, Serialize, Deserialize, Getters)]
#[serde(default)]
pub struct EditCommand {
    command: String,
    args: Vec<String>,
    inherit_shell: bool,
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

    pub fn run_command(
        &self,
        insert: Option<&str>,
    ) -> Result<Result<Either<ExitStatus, Output>, std::io::Error>, String> {
        let mut command = self.to_command(insert)?;
        if self.inherit_shell {
            let result = command.status();
            Ok(result.map(|e| Either::Left(e)))
        } else {
            let result = command.output();
            Ok(result.map(|o| Right(o)))
        }
    }
}

impl Default for EditCommand {
    fn default() -> Self {
        #[cfg(unix)]
        return EditCommand {
            command: "code".to_owned(),
            args: vec!["-w".to_owned(), "$$$".to_owned()],
            inherit_shell: false,
        };
        #[cfg(windows)]
        return EditCommand {
            command: "code.cmd".to_owned(),
            args: vec!["-w".to_owned(), "$$$".to_owned()],
        };
    }
}
