use crate::tokenizer::{Token, TokenType};

pub fn preprocessor_literal<'src>(
    curr: &mut usize,
    file: &'src str,
    bytes: &[u8],
    tokens: &mut Vec<Token<'src>>,
) {
    let start = *curr;
    while *curr < bytes.len() {
        if matches!(bytes[*curr], b' ' | b'\t' | b'\n' | b'\r' | b'"' | b'<') {
            break;
        }
        *curr += 1;
    }

    let directive = &file[start..*curr];
    tokens.push(Token::new(TokenType::Preprocessor, directive));

    if directive == "#include" {
        header_file(curr, file, bytes, tokens);
    }
}

fn header_file<'src>(
    curr: &mut usize,
    file: &'src str,
    bytes: &[u8],
    tokens: &mut Vec<Token<'src>>,
) {
    let ws_start = *curr;
    while *curr < bytes.len() && matches!(bytes[*curr], b' ' | b'\t') {
        *curr += 1;
    }

    if *curr > ws_start {
        tokens.push(Token::new(TokenType::Whitespace, &file[ws_start..*curr]));
    }

    if *curr >= bytes.len() {
        return;
    }

    let closing = match bytes[*curr] {
        b'"' => b'"',
        b'<' => b'>',
        _ => return,
    };

    let start = *curr;
    *curr += 1;
    while *curr < bytes.len() && bytes[*curr] != closing {
        *curr += 1;
    }
    if *curr < bytes.len() {
        *curr += 1;
    }

    tokens.push(Token::new(TokenType::HeaderFile, &file[start..*curr]));
}
