use crate::shared::ctrl_c_handler::{clear_ctrl_c_handler, set_ctrl_c_handler};
use crate::shared::util::command_fmt::command_to_string;
use std::cell::RefCell;
use std::process::{Child, Command, ExitStatus};
use std::rc::Rc;
use std::thread;
use std::time::Duration;

thread_local! {
    static CURRENT_COMMAND: RefCell<Option<Rc<RefCell<Child>>>> = const { RefCell::new(None) };
}

pub trait InterruptableCommand {
    fn run_interruptable(&mut self) -> Result<ExitStatus, String>;
}

impl InterruptableCommand for Command {
    fn run_interruptable(&mut self) -> Result<ExitStatus, String> {
        let child = Rc::new(RefCell::new(self.spawn().map_err(|e| {
            format!(
                "E88 Error running command `{}`: {}",
                command_to_string(self),
                e
            )
        })?));

        CURRENT_COMMAND.with(|ch| ch.borrow_mut().replace(child.clone()));

        set_ctrl_c_handler(Box::new(|| {
            CURRENT_COMMAND.with(|ch| {
                if let Some(cmd) = ch.borrow_mut().take() {
                    let _ = cmd.borrow_mut().kill();
                }
            });
        }));

        let ret = loop {
            match child.borrow_mut().try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) => thread::sleep(Duration::from_millis(100)),
                Err(e) => {
                    clear_ctrl_c_handler();
                    return Err(format!(
                        "E89 Error polling command `{}` for completion: {}",
                        command_to_string(self),
                        e
                    ));
                }
            }
        };

        clear_ctrl_c_handler();

        println!();

        Ok(ret)
    }
}
