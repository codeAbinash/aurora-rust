use crate::tokenizer::{nth_char, Token, TokenType};

pub fn escape_sequence(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut escape = String::new();
    let c = nth_char(file, *curr);
    escape.push(c);

    *curr += 1;
    let next_char = nth_char(file, *curr);

    if is_escape_sequence(next_char) {
        escape.push(next_char);
        *curr += 1;
    }

    tokens.push(Token {
        token_type: TokenType::Escape,
        value: escape,
    });
}

pub fn is_escape_sequence(c: char) -> bool {
    match c {
        '\'' | '"' | '0' | '\\' | 'a' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => true,
        _ => false,
    }
}
