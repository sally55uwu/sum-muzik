/**
 * main.rs
 * Entry point
 */
mod player;
mod utils;

use player::{add_song_to_sink, play_music, provide_path};
// use player::update_master_volume;
use rodio::{OutputStream, Sink};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use utils::{clear_terminal, help, handle_invalid_cmd};

fn main() {
    // Initialize Sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let shared_sink = Arc::new(Mutex::new(sink));

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
                    // "delete" => println!("Deleting from playlist"),
                    "add" => {
                        let song_path = provide_path();
                        if let Err(e) = add_song_to_sink(Arc::clone(&shared_sink), song_path) {
                            eprintln!("Error adding song to playlist: {}", e);
                        }
                    }
                    "play" => {
                        let sink = shared_sink.lock().unwrap();
                        if sink.empty() {
                            let sink_clone = Arc::clone(&shared_sink);
                            let song_path = provide_path();

                            // Start music playback in the background
                            thread::spawn(move || {
                                if let Err(e) = play_music(sink_clone, song_path)
                                {
                                    eprintln!("Error playing music: {}", e);
                                }
                            });
                        } else {
                            println!("\nThere is one active player\n");
                            //println!(" Create a new player? (Y/N): ");
                        }
                    }
                    "pause" => shared_sink.lock().unwrap().pause(),
                    "resume" => shared_sink.lock().unwrap().play(),
                    "vu" => {
                        let sink = shared_sink.lock().unwrap();
                        let sink_volume = sink.volume();
                        if sink_volume < 1.0 {
                            let  mut new_volume = sink_volume + default_volume_offset;
                            new_volume = new_volume.clamp(0.0, 1.0);
                            sink.set_volume(new_volume);
                            println!("Volume increased to {}", format!("{:.0}", new_volume * 100.0));
                        }
                    }
                    "vd" => {
                        let sink = shared_sink.lock().unwrap();
                        let sink_volume = sink.volume();
                        if sink_volume > 0.0 {
                            let  mut new_volume = sink_volume - default_volume_offset;
                            new_volume = new_volume.clamp(0.0, 1.0);
                            sink.set_volume(new_volume);
                            println!("Volume decreased to {}", format!("{:.0}", new_volume * 100.0));
                        }
                    }
                    // "vu" => update_master_volume(true, Some(0.1)),
                    // "vd" => update_master_volume(false, Some(0.1)),
                    "clear" => clear_terminal(),
                    "help" => help(),
                    "exit" => break,
                    _ => handle_invalid_cmd(input),
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
