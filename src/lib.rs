#[path = "lib/comments.rs"]
mod comments;

#[path = "lib/preprocessor.rs"]
mod preprocessor;

mod highlighter;

mod tokenizer;

use colored::Colorize;
use highlighter::highlighter;
use tokenizer::engine;

pub fn aurora(file_name: &String) {
    let file = open_file(file_name);

    match file {
        Ok(file) => engine(&file),
        Err(_) => println!(
            "Cannot read `{}` file. Make sure the file exists and you have read permission.",
            file_name.red()
        ),
    }
}

fn open_file(file_name: &String) -> std::io::Result<String> {
    let file = std::fs::read_to_string(file_name)?;
    Ok(file)
}
