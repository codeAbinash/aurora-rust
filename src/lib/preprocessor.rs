use crate::tokenizer::{nth_char, Token, TokenType};

pub fn preprocessor_literal(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    value.push('#');
    *curr += 1;
    let mut c = nth_char(file, *curr);

    while *curr < file.len() && c != ' ' {
        value.push(c);
        *curr += 1;
        c = nth_char(file, *curr);
    }

    tokens.push(Token {
        token_type: TokenType::Preprocessor,
        value,
    });

    // If the preprocessor is `#include` then it is a header file
    let value = tokens.last().unwrap().value.clone();
    if value == "#include" {
        header_file(curr, file, tokens);
    }
}

fn header_file(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {}
