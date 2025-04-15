use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use std::io::BufReader;
use std::process;
use std::io::{self, Write};

/// Prompts user to provide the absolute path (String) of a song.
//
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

pub fn add_to_playlist(
    song: PathBuf,
    shared_playlist: &mut Vec<PathBuf>)
-> Result<(), Box<dyn Error>> {
    let _song_exists = song.exists();
    shared_playlist.push(song);

    Ok(())

    }

pub fn list_songs(
    shared_playlist: &mut Vec<PathBuf>){

    if shared_playlist.len() < 1 {
        println!("No songs in current playlist");
    }

    if shared_playlist.len() == 1{
        let song_name = shared_playlist[0].clone().into_os_string().into_string().unwrap();

        println!("[0] {}", song_name);
    }
    else{
        let mut list_num = 0;
        for x in shared_playlist.iter(){
            println!("[{}] {}", list_num, 
                     x.clone().into_os_string().into_string().unwrap());
            list_num = list_num + 1;
        }
    }

}

pub fn remove_from_playlist(
    shared_playlist: &mut Vec<PathBuf>){

    if shared_playlist.len() == 0 {
        println!("No songs to remove");
    }

    list_songs(shared_playlist);

    println!("Input the number of the corresponding song you wish to remove");

    let mut input = String::new();
    let _cmd_input = io::stdin().read_line(&mut input);

    let my_integer = input.trim().parse().expect("Failed to parse string to integer");
    
    let removed = shared_playlist.remove(my_integer);

    println!("{}", removed.into_os_string().into_string().unwrap());

}
