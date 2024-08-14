#[cfg(test)]
mod tests {
    use crate::{
        error_handling::error::{AppError, AppErrorCodes},
        list::Command,
        validate_input,
    };
    use std::io;

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
        let result: Result<_, AppError> = validate_input(&invalid_input);
        let expected_result: Result<_, AppError> = Err(AppError::new(AppErrorCodes::E1001, None));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn fromstr_trait_impl_for_command_success() {
        let exit_command_value: &str = "ExIt";

        let result: Command = exit_command_value.parse::<Command>().unwrap();
        let expected_result: Command = Command::Exit;

        assert_eq!(result, expected_result);
    }

    #[test]
    fn fromstr_trait_impl_for_command_fail() {
        let unrecognized_command: &str = "";

        let result: Result<_, AppError> = unrecognized_command.parse::<Command>();
        let expected_result: Result<_, AppError> = Err(AppError::new(AppErrorCodes::E1002, None));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn debug_trait_impl_for_command() {
        let exit_command: Command = Command::Exit;

        let result: String = format!("{:?}", exit_command);
        let expected_result: String = String::from("Command::Exit");

        assert_eq!(result, expected_result);
    }
}
