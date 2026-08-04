use crate::tokenizer::{nth_byte, Token, TokenType};

pub fn number_literal<'src>(
    curr: &mut usize,
    file: &'src str,
    bytes: &[u8],
    tokens: &mut Vec<Token<'src>>,
) {
    let current_char = bytes[*curr];
    let next_char = nth_byte(bytes, *curr + 1);

    if current_char == b'0' && matches!(next_char, b'x' | b'X') {
        scan_number(
            curr,
            file,
            bytes,
            TokenType::HexPrefix,
            TokenType::HexLiteral,
            tokens,
            u8::is_ascii_hexdigit,
        );
    } else if current_char == b'0' && matches!(next_char, b'b' | b'B') {
        scan_number(
            curr,
            file,
            bytes,
            TokenType::BinPrefix,
            TokenType::BinLiteral,
            tokens,
            |b| matches!(b, b'0' | b'1'),
        );
    } else if current_char == b'0' && matches!(next_char, b'0'..=b'7') {
        scan_number(
            curr,
            file,
            bytes,
            TokenType::OctPrefix,
            TokenType::OctLiteral,
            tokens,
            |b| matches!(b, b'0'..=b'7'),
        );
    } else {
        let start = *curr;
        let mut dots = 0;

        while *curr < bytes.len() && (bytes[*curr].is_ascii_digit() || bytes[*curr] == b'.') {
            if bytes[*curr] == b'.' {
                dots += 1;
            }
            *curr += 1;
        }

        let token_type = if dots > 1 {
            TokenType::Unknown
        } else {
            TokenType::Number
        };
        tokens.push(Token::new(token_type, &file[start..*curr]));
    }
}

fn scan_number<'src>(
    curr: &mut usize,
    file: &'src str,
    bytes: &[u8],
    prefix: TokenType,
    literal: TokenType,
    tokens: &mut Vec<Token<'src>>,
    predicate: impl Fn(&u8) -> bool,
) {
    tokens.push(Token::new(prefix, &file[*curr..*curr + 2]));
    *curr += 2; // Skip the prefix

    let start = *curr;
    while *curr < bytes.len() && predicate(&bytes[*curr]) {
        *curr += 1;
    }

    tokens.push(Token::new(literal, &file[start..*curr]));
}
