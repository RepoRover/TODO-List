use std::io;

use todo_list_lib::validate_input;

fn main() {
    start();
}

/*
`start` function returns a list of already existing TODOs
and initiates the program execution process
*/
fn start() {
    // TODO: return all the existing TODOs before handling any inputs

    // For each incoming input line run the app
    for input in io::stdin().lines() {
        match validate_input(&input) {
            Ok(c) => c.handle(),
            Err(e) => {
                // TODO: use custom error handling when developed

                eprintln!("{}", e);
                continue;
            }
        };
    }
}
