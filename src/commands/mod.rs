use core::fmt;
use std::fmt::{Debug, Display, Formatter};

pub mod new_command;
pub mod help_command;

#[derive(Debug)]
pub struct CommandError {
    message: String
}

impl std::error::Error for CommandError {}

impl CommandError {
    pub fn unknown_command(issued_command: Option<&str>) -> CommandError {
        match issued_command {
            Some(command_name) => {
                CommandError { 
                    message: format!("Unknown command {}. Use 'next-butler help' to see what you can do", command_name)
                }
            },
            None => {
                CommandError { 
                    message: String::from("Unknown command {}. Use 'next-butler help' to see what you can do")
                }
            }
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[COMMAND ERROR] {}", &self.message)
    }
}