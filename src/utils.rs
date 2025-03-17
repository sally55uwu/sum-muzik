/**
 * utils.rs
 * Utility Functions
 */
use std::process::Command;

/// Clears the terminal using appropriate command for Windows or Unix
pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
