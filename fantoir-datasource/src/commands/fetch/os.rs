//! OS-related helper methods

use std::env::consts::OS;
use std::process::{Command, Stdio};

pub fn is_command_available (command: &str) -> bool {
    let command_to_use = match OS {
        "windows" => "where",
        _ => "which", // command -v is sometimes recommended, but doesn't exist as standalone
    };

    // Use the exit code to determine if the command has been found
    Command::new(command_to_use)
        .arg(command)
        .stdout(Stdio::null()) // Discard both stdout and stderr
        .stderr(Stdio::null())
        .status()
        .expect("failed to execute process")
        .success()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_command_available () {
        assert!(is_command_available("unzip"));
        assert!(!is_command_available("notexisting"));
    }
}
