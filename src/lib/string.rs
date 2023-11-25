use crate::{
    escape::escape_sequence,
    tokenizer::{nth_char, Token, TokenType},
};

pub fn character_literal(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    value.push('\'');
    *curr += 1;
    let mut c = nth_char(file, *curr);

    while *curr < file.len() && c != '\'' {
        // Escape sequence
        if c == '\\' {
            tokens.push(Token {
                token_type: TokenType::String,
                value: value,
            });
            escape_sequence(curr, file, tokens);
            value = String::new(); // Reset value
            c = nth_char(file, *curr);
            continue;
        }

        value.push(c);
        *curr += 1;
        c = nth_char(file, *curr);
    }

    value.push('\'');
    *curr += 1;
    tokens.push(Token {
        token_type: TokenType::String,
        value,
    });
}
