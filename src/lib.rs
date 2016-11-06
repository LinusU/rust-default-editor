use std::env;
use std::env::VarError;

/// Get the default editor for the current environment
pub fn get() -> Result<String, VarError> {
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

    Ok("vi".to_string())
}

#[cfg(test)]
mod tests {
    use std::env;

    fn it_falls_back_to_vi() {
        env::remove_var("VISUAL");
        env::remove_var("EDITOR");

        assert_eq!(super::get(), Ok("vi".to_string()));
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

    #[test]
    fn all_tests() {
        // Wrap all tests in another function since they cannot be run in parallel

        it_falls_back_to_vi();
        it_returns_visual();
        it_returns_editor();
        it_returns_visual_before_editor();
    }
}
