use crate::commands::{execute_command, is_command};
use aurora::aurora;
use colored::*;
use std::env;

mod commands;
mod info;

fn main() {
    let args: Vec<String> = env::args().collect();

    // If there is no argument
    if args.len() == 1 {
        println!("{}", "Please provide a file name as an argument".red());
        println!();
        return;
    }

    let first_arg = &args[1];

    if is_command(&first_arg) {
        execute_command(first_arg)
    } else {
        aurora(first_arg);
    }
}
