use derive_getters::Getters;
use either::{Either, Right};
use serde::{Deserialize, Serialize};
use std::process::{Command, ExitStatus, Output};

const PATH_REPLACE_ARG: &str = "$dir$";

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
            if arg == PATH_REPLACE_ARG {
                if let Some(insert) = insert {
                    command.arg(insert);
                    inserted = true;
                } else {
                    return Err(format!(
                        "E01 Found `{PATH_REPLACE_ARG}` in command with no insertion"
                    ));
                }
            } else {
                command = command.arg(arg);
            }
        }

        if insert.is_some() && !inserted {
            return Err(format!(
                "E04 One argument must be `{PATH_REPLACE_ARG}` for directory argument insertion. Check your config file."
            ));
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
            Ok(result.map(Either::Left))
        } else {
            let result = command.output();
            Ok(result.map(Right))
        }
    }
}

impl Default for EditCommand {
    fn default() -> Self {
        #[cfg(unix)]
        return EditCommand {
            command: "code".to_owned(),
            args: vec!["-w".to_owned(), PATH_REPLACE_ARG.to_owned()],
            inherit_shell: false,
        };
        #[cfg(windows)]
        return EditCommand {
            command: "code.cmd".to_owned(),
            args: vec!["-w".to_owned(), PATH_REPLACE_ARG.to_owned()],
        };
    }
}
