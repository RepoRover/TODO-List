use std::{fmt::Debug, process, str::FromStr};

// TODO: implemet test for each case
#[derive(PartialEq)]
pub enum Command {
    Exit,
}

impl Command {
    // TODO: implemet test
    pub fn handle(&self) {
        match self {
            Command::Exit => {
                println!("Exiting the application");
                process::exit(1);
            }
        }
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

// TODO: implemet test
impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Exit => f.write_str("Command::Exit"),
        }
    }
}
