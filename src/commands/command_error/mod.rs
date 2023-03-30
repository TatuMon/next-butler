use core::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct CommandError {
    pub message: String
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
                    message: String::from("Unknown command. Use 'next-butler help' to see what you can do")
                }
            }
        }
    }

    pub fn wrong_location() -> CommandError {
        CommandError {
            message: String::from("Can't run next-butler from outside of a next project's root folder")
        }
    }

    pub fn invalid_file_type() -> CommandError {
        CommandError {
            message: String::from("Invalid file type. Use 'next-butler new --help' to see which are valid")
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}
