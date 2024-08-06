use std::{io::Error, process};

mod tests;

pub fn handle_input(input: Result<String, Error>) {
    let value: String = validate_input(input);

    // match value.as_str() {
    //     "exit" => process::exit(1),
    //     _ => println!("Unrecognized command"),
    // }
}

/*
Takes ownership of the passed in 'input' and returns trimmed lowercase value
*/
fn validate_input(input: Result<String, Error>) -> String {
    match input {
        Ok(v) => v.trim().to_lowercase(),
        Err(_) => process::exit(1),
    }
}
