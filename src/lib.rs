pub mod error_handling;
pub mod list;
mod tests;

use std::io;

use error_handling::error::{AppError, AppErrorCodes};
use list::Command;

pub fn check_app_health() {
    // TODO: check if the log files exist
}

/*
Takes entered input, converts it to Command enum and returns the command
*/
pub fn validate_input<'a>(input: &'a Result<String, io::Error>) -> Result<Command, AppError<'a>> {
    match input {
        Ok(value) => value.parse::<Command>(),
        Err(e) => {
            let err: AppError = AppError::new(AppErrorCodes::E1001, Some(Box::new(e)));
            Err(err)
        }
    }
}
