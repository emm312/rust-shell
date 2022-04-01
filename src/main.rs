use std::io;
use std::io::{Write};
mod filesystemhandler;
extern crate serde;
use serde_json;
use std::fs::File;
use std::io::Read;

fn main() {
    // read the json file
    let mut filesystemobj = filesystemhandler::FileSystem::new("/".to_string());
    let mut file = File::open(r"src\filesystem.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    filesystemobj = serde_json::from_str(&contents).unwrap();
    


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
                    println!("Available commands:\n\tclear\n\texit\n\thelp\n\tmkdir\n\tmkfile\n\tls\n\tlsdebug");
                    break;
                },
                "mkdir" => {
                    filesystemobj.add_folder(input().trim_end().to_string());
                    let v = serde_json::to_string(&filesystemobj).unwrap();
                    save_filesystem(&filesystemobj);
                    break;
                },
                "mkfile" => {
                    filesystemobj.add_file(input().trim_end().to_string());
                    let v = serde_json::to_string(&filesystemobj).unwrap();
                    save_filesystem(&filesystemobj);
                    break;
                },
                "ls" => {
                    println!("Folders: {:?}", filesystemobj.folders);
                    println!("Files: {:?}", filesystemobj.files);
                    break;
                },
                "lsdebug" => {
                    println!("{}", serde_json::to_string(&filesystemobj).unwrap());
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

fn save_filesystem(filesystemobj: &filesystemhandler::FileSystem) -> bool {
    let v = serde_json::to_string(&filesystemobj).unwrap();
    let mut file = File::create(r"src\filesystem.json").unwrap();
    file.write_all(v.as_bytes()).unwrap();
    return true;
}