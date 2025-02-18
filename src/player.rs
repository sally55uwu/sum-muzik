/**
 * player.rs
 * Audio Playback Handler
 */
use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::process;
use std::sync::{Arc, Mutex};

#[cfg(target_os = "windows")]
use windows_volume_control::AudioController;

// Start of Windows Volume Control ---------------------------------------------

#[cfg(target_os = "windows")]
// Wrapper Struct that wraps around AudioController
pub struct SafeAudioController {
    controller: AudioController,
}

#[cfg(target_os = "windows")]
// Allows safe management of unsafe code by encapsulating interactions
// with `windows_volume_control` crate and gracefully handling errors.
impl SafeAudioController {
    // Constructor - initialize AudioController safely and default sessions
    pub fn new() -> Result<Self, String> {
        unsafe {
            let mut controller = AudioController::init(None);
            controller.GetSessions();
            controller.GetDefaultAudioEnpointVolumeControl();
            controller.GetAllProcessSessions();
            Ok(SafeAudioController { controller })
        }
    }

    // Get current volume of master audio session
    // Master session: main audio control for entire system's output
    pub fn get_master_volume(&mut self) -> Result<f32, String> {
        unsafe {
            if let Some(session) = self.controller.get_session_by_name("master".to_string()) {
                Ok(session.getVolume())
            } else {
                Err("Master session not found".to_string())
            }
        }
    }

    // Set the volume of the master audio session
    pub fn set_master_volume(&mut self, volume: f32) -> Result<(), String> {
        unsafe {
            if let Some(session) = self.controller.get_session_by_name("master".to_string()) {
                session.setVolume(volume);
                Ok(())
            } else {
                Err("Master session not found".to_string())
            }
        }
    }
}

#[cfg(target_os = "windows")]
// High-level entry point that updates the volume of the master audio session
// given the type and value of the volume change
pub fn update_master_volume(volume_up: bool, volume_change: Option<f32>) {
    // Initialize SafeAudioController
    let mut audio_controller = match SafeAudioController::new() {
        Ok(controller) => controller,
        Err(err) => {
            eprintln!("Failed to initialize AudioController: {}", err);
            return; // Exit
        }
    };

    let volume_change = volume_change.unwrap_or(0.1); // Default value is 0.1

    // Get current master volume
    match audio_controller.get_master_volume() {
        Ok(curr_volume) => {
            // Update volume appropriately and clamp its value
            let new_volume = if volume_up {
                (curr_volume + volume_change).clamp(0.0, 1.0)
            } else {
                (curr_volume - volume_change).clamp(0.0, 1.0)
            };

            // Set new volume
            if let Err(err) = audio_controller.set_master_volume(new_volume) {
                eprintln!("Failed to set volume: {}", err);
            } else {
                println!(
                    "Volume {} by {:.2}. New volume: {:.2}",
                    if volume_up { "increased" } else { "decreased" },
                    volume_change,
                    new_volume
                );
            }
        }
        Err(err) => {
            eprintln!("Error retrieving master volume: {}", err);
        }
    }
}

// End of Windows Volume Control -----------------------------------------------

/// Prompts user to provide the absolute path (String) of a song.
///
/// # Returns
/// Provided path (PathBuf).
pub fn provide_path() -> PathBuf {
    println!("Provide the absolute path where the song is:");

    // Read line from standard input and store trimmed line as path
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let path = PathBuf::from(&line.trim());

    if !path.exists() {
        println!("File does not exist");
        process::exit(1);
    }

    path
}

/// Plays an audio file given its absolute path.
///
/// # Arguments
/// * `play_state`: Controls playback (Arc<Mutex<(bool, f32, bool)>>)
/// * `song`: The location of a song (PathBuf)
///
/// # Returns
/// - `Ok(())` if audio file plays.
/// - `Box<dyn Error>` if stream initialization, file opening,
///     or audio decoding raise an error
pub fn play_music(
    play_state: Arc<Mutex<(bool, f32, bool)>>,
    song: PathBuf,
) -> Result<(), Box<dyn Error>> {
    // Create audio output stream and sink for managing playback. Open song file
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let file = File::open(song)?;

    // Apply initial volume
    {
        let state = play_state.lock().unwrap();
        sink.set_volume(state.1);
    }

    // Start playback
    sink.append(Decoder::new(BufReader::new(file))?);

    // Mark playback as active
    {
        let mut state = play_state.lock().unwrap();
        state.0 = true;
    }

    // Update volume if requested during playback
    let mut previous_volume = {
        let state = play_state.lock().unwrap();
        state.1
    };

    // During playback
    while !sink.empty() {
        let state = play_state.lock().unwrap();

        let current_volume = state.1;

        // Only update volume if requested
        if current_volume != previous_volume {
            sink.set_volume(current_volume);
            previous_volume = current_volume;
        }

        // Pause or resume playback if requested
        if state.2 {
            sink.pause();
        } else {
            sink.play();
        }
    }

    // Mark playback as finished
    {
        let mut state = play_state.lock().unwrap();
        state.0 = false;
    }

    Ok(())
}
