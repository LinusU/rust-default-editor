use std::env;
use std::env::VarError;
use std::process::Command;

const FALLBACK_EDITOR: &str = "vi";

fn parse_command_program_with_args(command: &str) -> (String, Vec<String>) {
    let mut args = command.split_whitespace();
    let program = args.next().unwrap_or(FALLBACK_EDITOR).to_string();

    let args = args.map(|arg| arg.to_string()).collect();
    (program, args)
}

fn get_editor_command() -> Result<String, VarError> {
    match env::var("VISUAL") {
        Ok(result) => return Ok(result),
        Err(VarError::NotPresent) => {},
        Err(error) => return Err(error),
    }

    match env::var("EDITOR") {
        Ok(result) => return Ok(result),
        Err(VarError::NotPresent) => {},
        Err(error) => return Err(error),
    }

    Ok(FALLBACK_EDITOR.to_string())
}

/// Get the default editor for the current environment
pub fn get() -> Result<String, VarError> {
    match get_editor_command() {
        Ok(command) => Ok(command),
        Err(error) => Err(error),
    }
}

/// Get the default editor for the current environment and its arguments
pub fn get_with_args() -> Result<(String, Vec<String>), VarError> {
    match get_editor_command() {
        Ok(command) => Ok(parse_command_program_with_args(&command)),
        Err(error) => Err(error),
    }
}

/// Get the default editor for the current environment as a `Command`
pub fn editor() -> Command {
    let (program, args) = get_with_args().unwrap();

    let mut command = Command::new(program);
    if args.len() > 0 {
        command.args(args);
    }

    command
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::FALLBACK_EDITOR;

    fn it_falls_back_to_vi() {
        env::remove_var("VISUAL");
        env::remove_var("EDITOR");

        assert_eq!(super::get(), Ok(FALLBACK_EDITOR.to_string()));
    }

    fn it_returns_visual() {
        env::set_var("VISUAL", "test1");
        env::remove_var("EDITOR");

        assert_eq!(super::get(), Ok("test1".to_string()));
    }

    fn it_returns_editor() {
        env::remove_var("VISUAL");
        env::set_var("EDITOR", "test2");

        assert_eq!(super::get(), Ok("test2".to_string()));
    }

    fn it_returns_visual_before_editor() {
        env::set_var("VISUAL", "test3");
        env::set_var("EDITOR", "test4");

        assert_eq!(super::get(), Ok("test3".to_string()));
    }

    fn it_returns_editor_with_args() {
        env::remove_var("VISUAL");
        env::set_var("EDITOR", "test5 -a -b -c");

        assert_eq!(
            super::get_with_args(),
            Ok((
                "test5".to_string(),
                vec!["-a".to_string(), "-b".to_string(), "-c".to_string()]
            ))
        );
    }

    fn it_returns_editor_command_with_args () {
        env::remove_var("VISUAL");
        env::set_var("EDITOR", "test6 -a -b -c");

        let command = super::editor();
        assert_eq!(command.get_program(), "test6");
        let args: Vec<&std::ffi::OsStr> = command.get_args().collect();
        assert_eq!(args, &["-a", "-b", "-c"]);
    }

    fn it_returns_editor_command_without_args() {
        env::remove_var("VISUAL");
        env::set_var("EDITOR", "test7");

        let command = super::editor();
        assert_eq!(command.get_program(), "test7");
        let args: Vec<&std::ffi::OsStr> = command.get_args().collect();
        assert_eq!(args.len(), 0);
    }

    #[test]
    fn all_tests() {
        // Wrap all tests in another function since they cannot be run in parallel

        it_falls_back_to_vi();
        it_returns_visual();
        it_returns_editor();
        it_returns_visual_before_editor();
        it_returns_editor_with_args();
        it_returns_editor_command_with_args();
        it_returns_editor_command_without_args();
    }
}
