use crate::tokenizer::{Token, TokenType};

pub fn escape_sequence<'src>(
    curr: &mut usize,
    file: &'src str,
    bytes: &[u8],
    tokens: &mut Vec<Token<'src>>,
) {
    let start = *curr; // points at '\'
    *curr += 1;

    if *curr < bytes.len() && is_escape_char(bytes[*curr]) {
        *curr += 1;
    }

    tokens.push(Token {
        token_type: TokenType::Escape,
        value: &file[start..*curr],
    });
}

#[inline(always)]
pub fn is_escape_char(c: u8) -> bool {
    matches!(
        c,
        b'\'' | b'"' | b'0' | b'\\' | b'a' | b'b' | b'f' | b'n' | b'r' | b't' | b'v'
    )
}
