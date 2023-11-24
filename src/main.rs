use crate::commands::{execute_command, is_command};
use aurora::aurora;
use colored::*;
use std::env;

mod commands;
mod info;

fn main() {
    let args: Vec<String> = env::args().collect();

    // println!(
    //     "{}",
    //     "----------------------------------------".truecolor(10, 255, 100)
    // );

    // If there is no argument
    if args.len() == 1 {
        print!("Please provide a file name as an argument");
        return;
    }

    let first_arg = &args[1];

    // First argument may be a command or a filename to open

    if is_command(&first_arg) {
        execute_command(first_arg)
    } else {
        aurora(first_arg);
    }
}
