//Command line arguments in rust

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("Command: {:?}", command);

    if command == "hello" {
        println!("recevied hello command");
    } else if command == "status" {
        println!("received command status");
    } else {
        println!("unknown command");
    }
}