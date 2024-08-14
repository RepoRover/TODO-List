use std::{fmt::Debug, process, str::FromStr};

use crate::error_handling::error::{AppError, AppErrorCodes};

#[derive(PartialEq)]
pub enum Command {
    Exit,
}

impl Command {
    // NOTE: no tests written
    pub fn handle(&self) {
        match self {
            Command::Exit => Self::exit_app(),
        }
    }

    // NOTE: no tests written
    fn exit_app() {
        println!("Exiting the application");
        process::exit(0);
    }
}

/*
Converts &str into "Command" enumeration
*/
impl FromStr for Command {
    type Err = AppError<'static>; // NOTE: Can use static here ONLY if from_str method returns AppError with original_error as None!

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        match s.to_lowercase().trim() {
            "exit" => Ok(Command::Exit),
            _ => Err(AppError::new(AppErrorCodes::E1002, None)),
        }
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Exit => f.write_str("Command::Exit"),
        }
    }
}
