use std::error::Error;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct AppError<'a> {
    code: u16,
    utc_time: DateTime<Utc>,
    message: String,
    dev_description: String,
    original_error: Option<Box<dyn Error + 'a>>,
}

pub enum AppErrorCodes {
    E1001,
    E1002,
}

impl<'a> AppError<'a> {
    pub fn new(code: AppErrorCodes, original_error: Option<Box<dyn Error + 'a>>) -> AppError<'a> {
        let time: DateTime<Utc> = Utc::now();
        match code {
            AppErrorCodes::E1001 => AppError {
                code: 1001,
                utc_time: time,
                message: "Invalid input".to_string(),
                dev_description: "Error could occurre if something went wrong with an input"
                    .to_string(),
                original_error,
            },
            AppErrorCodes::E1002 => AppError {
                code: 1002,
                utc_time: time,
                message: "/-----\n\nUnrecognized command\n\n\\-----\n".to_string(),
                dev_description: "Error occured because user entered an invalid command"
                    .to_string(),
                original_error,
            },
        }
    }

    pub fn log(&self) {}
}

impl<'a> PartialEq for AppError<'a> {
    fn eq(&self, other: &Self) -> bool {
        // Exclude utc_time from equality comparison
        self.code == other.code
            && self.message == other.message
            && self.dev_description == other.dev_description
    }
}
