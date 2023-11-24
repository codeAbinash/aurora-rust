use colored::Colorize;

use crate::info::{self, VERSION};

pub fn is_command(command: &str) -> bool {
    // command.starts_with("--") || command.starts_with("-")
    command.starts_with("--") || command.starts_with("-")
}

// A raw string literal for the help message

const PRODUCT_TITLE: &str =
    "Aurora : The fastest and feature-rich C code highlighting library for Terminal";

pub fn execute_command(command: &String) {
    match command.as_str() {
        "-h" | "-help" | "--help" => help_command(), 
        "-v" | "-version" | "--version" => version_command(),
        "-about" | "--about" => about_command(),
        _ => println!(
            "`{}` is not a valid command!\nUse `aurora --help` to see the list of available commands.",
            command.bright_black()
        ),
    }
}

fn help_command() {
    aurora_terminal();
    println!(" {}", VERSION.truecolor(10, 230, 100));
    println!();
    println!("Usage");

    println!(
        "  {} {} ",
        "aurora".truecolor(10, 230, 100),
        "fileName".truecolor(63, 152, 238)
    );
    println!();
    println!("Example");
    println!(
        "  {} {} ",
        "aurora".truecolor(10, 230, 100),
        "main.c".truecolor(63, 152, 238)
    );
    println!();
    println!("Options");
    println!(
        "  {} {}",
        "-h, -help, --help".truecolor(233, 54, 90),
        "\t\tPrint this help message"
    );
    println!(
        "  {} {}",
        "-v, -version, --version".truecolor(233, 54, 90),
        "\tPrint version information"
    );
    println!(
        "  {} {}",
        "-about, --about".truecolor(233, 54, 90),
        "\t\tPrint about information"
    );

    println!();
}

pub fn version_command() {
    aurora_terminal();
    println!(" {}", VERSION.truecolor(10, 230, 100));
    println!();
    // println!("Check for updates: {}", info::REPO_URL.truecolor(63, 152, 238));
}

pub fn about_command() {
    // aurora();
    aurora_terminal();
    println!(" {}", VERSION.truecolor(10, 230, 100));
    println!();
    println!("{}", PRODUCT_TITLE);
    println!();
    println!("Author: {}", info::AUTHOR.truecolor(63, 152, 238));
    println!();
    println!(
        "Check for updates: {}",
        info::REPO_URL.truecolor(63, 152, 238)
    );
    println!();
}

pub fn _aurora() {
    println!(
        "{}",
        " ▄▄▄       █    ██  ██▀███   ▒█████   ██▀███   ▄▄▄      ".truecolor(210, 63, 195)
    );
    println!(
        "{}",
        "▒████▄     ██  ▓██▒▓██ ▒ ██▒▒██▒  ██▒▓██ ▒ ██▒▒████▄    ".truecolor(204, 60, 216)
    );
    println!(
        "{}",
        "▒██  ▀█▄  ▓██  ▒██░▓██ ░▄█ ▒▒██░  ██▒▓██ ░▄█ ▒▒██  ▀█▄  ".truecolor(174, 57, 213)
    );
    println!(
        "{}",
        "░██▄▄▄▄██ ▓▓█  ░██░▒██▀▀█▄  ▒██   ██░▒██▀▀█▄  ░██▄▄▄▄██ ".truecolor(169, 55, 233)
    );
    println!(
        "{}",
        " ▓█   ▓██▒▒▒█████▓ ░██▓ ▒██▒░ ████▓▒░░██▓ ▒██▒ ▓█   ▓██▒".truecolor(132, 55, 238)
    );
    println!(
        "{}",
        " ▒▒   ▓▒█░░▒▓▒ ▒ ▒ ░ ▒▓ ░▒▓░░ ▒░▒░▒░ ░ ▒▓ ░▒▓░ ▒▒   ▓▒█░".truecolor(113, 67, 245)
    );
    println!(
        "{}",
        "  ▒   ▒▒ ░░░▒░ ░ ░   ░▒ ░ ▒░  ░ ▒ ▒░   ░▒ ░ ▒░  ▒   ▒▒ ░".truecolor(70, 129, 242)
    );
    println!(
        "{}",
        "  ░   ▒    ░░░ ░ ░   ░░   ░ ░ ░ ░ ▒    ░░   ░   ░   ▒   ".truecolor(63, 152, 238)
    );
    println!(
        "{}",
        "      ░  ░   ░        ░         ░ ░     ░           ░  ░\n".truecolor(53, 175, 238)
    );
}

pub fn aurora_terminal() {
    print!("{}", "A".truecolor(233, 54, 90));
    print!("{}", "u".truecolor(226, 65, 135));
    print!("{}", "r".truecolor(219, 61, 166));
    print!("{}", "o".truecolor(210, 63, 195));
    print!("{}", "r".truecolor(204, 60, 216));
    print!("{}", "a".truecolor(174, 57, 213));
    print!(" ");
    print!("{}", "T".truecolor(169, 55, 233));
    print!("{}", "e".truecolor(132, 55, 238));
    print!("{}", "r".truecolor(113, 67, 245));
    print!("{}", "m".truecolor(70, 129, 242));
    print!("{}", "i".truecolor(63, 152, 238));
    print!("{}", "n".truecolor(53, 175, 238));
    print!("{}", "a".truecolor(45, 195, 236));
    print!("{}", "l".truecolor(41, 205, 236));
}
