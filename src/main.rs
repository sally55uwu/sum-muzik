/**
 * main.rs
 * Entry point
 */
mod player;
mod utils;
mod playlists;

use player::{add_song_to_sink, play_music, provide_path};
// use player::update_master_volume;
use rodio::{OutputStream, Sink};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use utils::{clear_terminal, help, handle_invalid_cmd};
use playlists::{remove_from_playlist, add_to_playlist, list_songs};  
use std::path::PathBuf;

fn main() {
    // Initialize Sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let shared_sink = Arc::new(Mutex::new(sink));

    let mut shared_playlist : Vec<PathBuf> = Vec::new();
    let mut playlists_pos = 0;

    let default_volume_offset = 0.1;

    let mut hit_play = false;
    
    loop {
        print!("♪ ");
        io::stdout().flush().unwrap(); // Display prompt immediately
    
        // If sink is empty and song(s) in the playlist, add one song to sink
        if shared_sink.lock().unwrap().empty() 
            && shared_playlist.len() > 1 
                && playlists_pos != shared_playlist.len()
        {
            playlists_pos = playlists_pos + 1;
            
            // println!("{}", playlists_pos);
            if let Err(e) = add_song_to_sink(Arc::clone(&shared_sink), shared_playlist[playlists_pos].clone()) {
                eprintln!("Error adding song to sink playlists: {}", e);
            }
        }

        let mut input = String::new();

        // Read a line from stdin
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let input = input.trim();
                if input.is_empty() {
                    continue; // Ignore empty lines
                }
                if shared_sink.lock().unwrap().empty() 
                && shared_playlist.len() > 1 
                    && playlists_pos != shared_playlist.len()
            {
                playlists_pos = playlists_pos + 1;
                
                // println!("{}", playlists_pos);
                if let Err(e) = add_song_to_sink(Arc::clone(&shared_sink), shared_playlist[playlists_pos].clone()) {
                    eprintln!("Error adding song to sink playlists: {}", e);
                }
            }
                match input {
                    "delete" => remove_from_playlist(&mut shared_playlist),

                    // playlist[] = queue of N songs
                    // sink = the playback of one song from the playlist[]
                    //do not allow user to do add before play
                    "add" => {
                        
                        if hit_play {
                            
                            let song_path = provide_path();
                            
                            if let Err(e) = add_to_playlist(song_path.clone(), &mut shared_playlist){
                                eprintln!("Error adding song to vector playlist: {}", e);
                            }
                        }else{
                            println!("please input the play command first");
                        }  

                    }

                    //TODO: Add the song to playlist
                    "play" => {
                        let sink = shared_sink.lock().unwrap();
                        if sink.empty() {
                            let sink_clone = Arc::clone(&shared_sink);
                            let song_path = provide_path();
                            
                            if let Err(e) = add_to_playlist(song_path.clone(), &mut shared_playlist){
                                eprintln!("Error adding song to vector playlist: {}", e);
                            }


                            // Start music playback in the background
                            thread::spawn(move || {
                                if let Err(e) = play_music(sink_clone, song_path)
                                {
                                    eprintln!("Error playing music: {}", e);
                                }
                            });
                            hit_play = true;
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
                    "list" => list_songs(&mut shared_playlist),
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
