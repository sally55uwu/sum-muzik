/**
 * main.rs
 * Entry point
 */
mod player;

use player::play_music;
// use player::update_master_volume;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "add" => println!("adding to playlist"),
            "delete" => println!("delete"),
            "play" => play_music().unwrap(),
            "pause" => println!("pause the song"),
            // "vu" => update_master_volume(true, Some(0.1)),
            // "vd" => update_master_volume(false, Some(0.1)),
            "exit" => println!("exit the app"),
            _ => println!("Invalid command provided"),
            /* "help" | "--help" | "-h" | _ => help(), */
        }
    } else {
        println!("No command provided");
    }
}
