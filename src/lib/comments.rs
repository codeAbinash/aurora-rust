use crate::tokenizer::{Token, TokenType};

pub fn single_line_comment<'src>(
    curr: &mut usize,
    file: &'src str,
    bytes: &[u8],
    tokens: &mut Vec<Token<'src>>,
) {
    let start = *curr - 1; // start of '//'
    while *curr < bytes.len() && bytes[*curr] != b'\n' {
        *curr += 1;
    }
    tokens.push(Token::new(TokenType::Comment, &file[start..*curr]));
}

pub fn multi_line_comment<'src>(
    curr: &mut usize,
    file: &'src str,
    _bytes: &[u8],
    tokens: &mut Vec<Token<'src>>,
) {
    let start = *curr - 1; // start of '/*'

    if let Some(end_idx) = file[*curr..].find("*/") {
        *curr += end_idx + 2;
    } else {
        *curr = file.len();
    }

    tokens.push(Token::new(TokenType::Comment, &file[start..*curr]));
}
