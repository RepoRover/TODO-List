use std::{
    io::{self, Error},
    process,
};

use todo_list_lib::{self, handle_input};

fn main() {
    start();
}

/*
`start` function returns a list of already existing TODOs
and initiates the program execution process
*/
fn start() {
    // TODO: return all the existing TODOs before handling any inputs

    for input in io::stdin().lines() {
        handle_input(input);
    }
}
