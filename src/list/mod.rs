use std::{fmt::Debug, process, str::FromStr};

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
        process::exit(1);
    }
}

/*
Converts &str into "Command" enumeration
*/
impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Command, Self::Err> {
        match s.to_lowercase().trim() {
            "exit" => Ok(Command::Exit),
            _ => {
                // TODO: use custom error handling when developed
                // unrecognized command etc.

                Err(String::from("/-----\n\nUnrecognized command\n\n\\-----\n"))
            }
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
