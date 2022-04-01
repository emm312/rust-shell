use std::io;
use std::io::{Write};
mod filesystemhandler;
extern crate serde;
use serde_json;

fn main() {

    let mut fs = filesystemhandler::FileSystem::new("/".to_string());


    loop {
        print!("rustOS@user:~$ ");
        io::stdout().flush();
        let command = input();
        let command_array = command.split_whitespace().collect::<Vec<&str>>();
        for s in command_array {
            match s {
                "exit" => {
                    println!("Exiting...");
                    return;
                },
                "clear" => {
                    println!("\x1B[2J\x1B[1;1H");
                    break;
                },
                "help" => {
                    println!("Available commands:\n\tclear\n\texit\n\thelp\n");
                    break;
                },
                "mkdir" => {
                    fs.add_folder(input().trim_end().to_string());
                    let v = serde_json::to_string(&fs).unwrap();
                    println!("{}", v);
                    break;
                },
                "mkfile" => {
                    fs.add_file(input().trim_end().to_string());
                    let v = serde_json::to_string(&fs).unwrap();
                    println!("{}", v);
                    break;
                },
                _ => {
                    println!("Command not found.\n");
                    break;
                }
            } 
        }
    }   
}


fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input;
}
