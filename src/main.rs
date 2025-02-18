/**
 * main.rs
 * Entry point
 */
mod player;

use player::{play_music, provide_path};
// use player::update_master_volume;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let play_state = Arc::new(Mutex::new((false, 1.0, false)));
    let default_volume_offset = 0.1;

    loop {
        print!("♪ ");
        io::stdout().flush().unwrap(); // Display prompt immediately

        let mut input = String::new();

        // Read a line from stdin
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue; // Ignore empty lines
                }

                match input {
                    "add" => println!("Adding to playlist"),
                    "delete" => println!("Deleting from playlist"),
                    "play" => {
                        let play_state_clone = Arc::clone(&play_state);
                        let song_path = provide_path();

                        // Start music playback in the background
                        thread::spawn(move || {
                            if let Err(e) = play_music(play_state_clone, song_path) {
                                eprintln!("Error playing music: {}", e);
                            }
                        });
                    }
                    "pause" => {
                        let mut state = play_state.lock().unwrap();
                        state.2 = !state.2;
                    }
                    "vu" => {
                        let mut state = play_state.lock().unwrap();
                        if state.1 < 1.0 {
                            state.1 += default_volume_offset;
                            state.1 = (state.1).clamp(0.0, 1.0);
                            println!("Volume increased to {}", format!("{:.0}", state.1 * 100.0));
                        }
                    }
                    "vd" => {
                        let mut state = play_state.lock().unwrap();
                        if state.1 > 0.0 {
                            state.1 -= default_volume_offset;
                            state.1 = (state.1).clamp(0.0, 1.0);
                            println!("Volume decreased to {}", format!("{:.0}", state.1 * 100.0));
                        }
                    }
                    // "vu" => update_master_volume(true, Some(0.1)),
                    // "vd" => update_master_volume(false, Some(0.1)),
                    "exit" => break,
                    _ => println!("Invalid command: {}", input),
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
