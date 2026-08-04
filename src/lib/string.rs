use crate::{
    escape::{escape_sequence, is_escape_char},
    tokenizer::{Token, TokenType},
};

pub fn character_literal<'src>(
    curr: &mut usize,
    file: &'src str,
    bytes: &[u8],
    tokens: &mut Vec<Token<'src>>,
) {
    scan_quoted(curr, file, bytes, tokens, b'\'');
}

pub fn string_literal<'src>(
    curr: &mut usize,
    file: &'src str,
    bytes: &[u8],
    tokens: &mut Vec<Token<'src>>,
) {
    scan_quoted(curr, file, bytes, tokens, b'"');
}

fn scan_quoted<'src>(
    curr: &mut usize,
    file: &'src str,
    bytes: &[u8],
    tokens: &mut Vec<Token<'src>>,
    quote: u8,
) {
    let mut start = *curr;
    *curr += 1; // Skip opening quote

    while *curr < bytes.len() && bytes[*curr] != quote {
        if bytes[*curr] == b'\\' && *curr + 1 < bytes.len() && is_escape_char(bytes[*curr + 1]) {
            tokens.push(Token::new(TokenType::String, &file[start..*curr]));
            escape_sequence(curr, file, bytes, tokens);
            start = *curr;
        } else {
            *curr += 1;
        }
    }

    if *curr < bytes.len() {
        *curr += 1;
    } // Skip closing quote
    tokens.push(Token::new(TokenType::String, &file[start..*curr]));
}
