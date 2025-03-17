/**
 * utils.rs
 * Utility Functions
 */
use std::process::Command;

const HELP_MESSAGE: &str = r#"USAGE:
    cargo run

COMMANDS:
    play      Play a given song
    add       Add song to queue
    pause     Pause audio playback
    resume    Resume paused audio playback
    vu        Increase volume
    vd        Decrease volume
    clear     Clear terminal
    help      Print this help message
    exit      Exit program"#;

/// Clears the terminal using appropriate command for Windows or Unix
pub fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/c").arg("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

/// Prints a help message
pub fn help() {
    println!("{}", HELP_MESSAGE);
}

// Prints an error message for an invalid command
pub fn handle_invalid_cmd(invalid_cmd: &str) {
    println!("Invalid command: {}\n", invalid_cmd);
    help();
}
