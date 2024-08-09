// Use 'pub' just to get rid of the: "unused import:" warning message
pub use super::*;
pub use crate::{list::Command, validate_input};

#[test]
fn validated_input_success() {
    // Simulate successful commands with mock inputs
    let exit_command_input: &Result<String, _> = &Ok("exit".to_string());
    let exit_command_expected: Command = Command::Exit;

    // Pass mock inputs to the validate_input function
    let exit_command_result: Command = validate_input(exit_command_input).unwrap();

    assert_eq!(
        exit_command_result, exit_command_expected,
        "Comparing '{:?}' with '{:?}'",
        exit_command_expected, exit_command_input
    );
}

#[test]
fn validated_input_fail() {
    // Simulate an error case by creating an Err variant
    let invalid_input: Result<_, io::Error> =
        Err(std::io::Error::new(std::io::ErrorKind::Other, "mock error"));

    // Call the validate_input function with the simulated error
    let result: Result<_, String> = validate_input(&invalid_input);
    let expected_result: Result<_, String> = Err(String::from("Invalid input"));

    assert_eq!(result, expected_result);
}
