pub mod list;
mod tests;

use std::io;

use list::Command;

/*
Takes entered input, converts it to Command enum and returns the command
*/
// TODO: implemet test
pub fn validate_input(input: &Result<String, io::Error>) -> Result<Command, String> {
    match input {
        Ok(value) => value.parse::<Command>(),
        Err(_) => Err(String::from("Invalid input")),
    }
}
