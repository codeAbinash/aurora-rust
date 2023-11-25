use crate::tokenizer::{nth_char, Token, TokenType};

pub fn number_literal(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    let mut c = nth_char(file, *curr);

    while *curr < file.len() && is_digit(c) {
        value.push(c);
        *curr += 1;
        c = nth_char(file, *curr);
    }

    tokens.push(Token {
        token_type: TokenType::Number,
        value,
    });
}

pub fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}
