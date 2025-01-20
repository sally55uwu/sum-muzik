use std::env;

fn main() {
   
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "add" => println!("adding to playlist"),
            "delete" => println!("delete"),  
            "play" => println!("play the song"),
            "pause" => println!("pause the song"),
            "vu" => println!("volume up"),
            "vd" => println!("volume down"),
            "exit" => println!("exit the app"),
            _ => println!("nothing was typed"),
            /* "help" | "--help" | "-h" | _ => help(), */
        }
    } else {
    }
}
