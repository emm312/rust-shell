use std::io;
use std::io::{Write};

fn main() {
    loop {
        print!("rustOS@user:~$ ");
        io::stdout().flush();
        let command = input();
        let command_array = command.split_whitespace();
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
