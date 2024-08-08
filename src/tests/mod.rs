// #[cfg(tests)]
// use super::*;
// use crate::{list::Command, validate_input};

// #[test]
// fn validate_commands_and_case_sensetivity() {
//     let exit_command_input: &Result<String, _> = &Ok("ExIt".to_string());
//     let exit_command = Command::Exit;

//     assert_eq!(
//         exit_command,
//         validate_input(exit_command_input).unwrap(),
//         "Comparing '{:?}' with '{:?}'",
//         exit_command,
//         exit_command_input
//     );
// }

// #[test]
// fn pass_invalid_command_as_input() {
//     let invalid_command: &Result<String, _> = &Ok("invalid".to_string());
//     let expected_result: Result<_, String> =
//         Err(String::from("/-----\n\nUnrecognized command\n\n\\-----\n"));

//     assert_eq!(
//         expected_result,
//         validate_input(invalid_command),
//         "Comparing '{:?}' with '{:?}'",
//         invalid_command,
//         expected_result
//     );
// }
