use crate::tokenizer::{nth_char, Token, TokenType};

pub fn number_literal(curr: &mut usize, chars: &[char], tokens: &mut Vec<Token>) {
    let c = nth_char(chars, *curr);
    let next = nth_char(chars, *curr + 1);

    if c == '0' {
        // Check if it is a HexLiteral
        if next == 'x' || next == 'X' {
            push_hex_prefix(curr, chars, tokens);
            hex_literal(curr, chars, tokens);
            return;
        }

        // Check if it is a BinLiteral
        if next == 'b' || next == 'B' {
            push_bin_prefix(curr, chars, tokens);
            bin_literal(curr, chars, tokens);
            return;
        }

        // Decimal Number Literal
        if next == '.' {
            decimal_number_literal(curr, chars, tokens);
            return;
        }

        // Check if it is an OctLiteral
        if next >= '0' && next <= '7' {
            push_oct_prefix(curr, chars, tokens);
            oct_literal(curr, chars, tokens);
            return;
        }

        decimal_number_literal(curr, chars, tokens);
    } else {
        decimal_number_literal(curr, chars, tokens);
    }
}

pub fn push_hex_prefix(curr: &mut usize, chars: &[char], tokens: &mut Vec<Token>) {
    let c = nth_char(chars, *curr);
    let next = nth_char(chars, *curr + 1);
    tokens.push(Token {
        token_type: TokenType::HexPrefix,
        value: format!("{}{}", c, next),
    });
    *curr += 2;
}

pub fn push_bin_prefix(curr: &mut usize, chars: &[char], tokens: &mut Vec<Token>) {
    let c = nth_char(chars, *curr);
    let next = nth_char(chars, *curr + 1);
    tokens.push(Token {
        token_type: TokenType::BinPrefix,
        value: format!("{}{}", c, next),
    });
    *curr += 2;
}

pub fn push_oct_prefix(curr: &mut usize, chars: &[char], tokens: &mut Vec<Token>) {
    let c = nth_char(chars, *curr);
    let next = nth_char(chars, *curr + 1);
    tokens.push(Token {
        token_type: TokenType::OctPrefix,
        value: format!("{}{}", c, next),
    });
    *curr += 2;
}

fn hex_literal(curr: &mut usize, chars: &[char], tokens: &mut Vec<Token>) {
    let mut value = String::new();
    let mut c = nth_char(chars, *curr);

    while *curr < chars.len() && is_hex_digit(c) {
        value.push(c);
        *curr += 1;
        c = nth_char(chars, *curr);
    }

    tokens.push(Token {
        token_type: TokenType::HexLiteral,
        value,
    });
}

fn bin_literal(curr: &mut usize, chars: &[char], tokens: &mut Vec<Token>) {
    let mut value = String::new();
    let mut c = nth_char(chars, *curr);

    while *curr < chars.len() && (c == '0' || c == '1') {
        value.push(c);
        *curr += 1;
        c = nth_char(chars, *curr);
    }

    tokens.push(Token {
        token_type: TokenType::BinLiteral,
        value,
    });
}

fn oct_literal(curr: &mut usize, chars: &[char], tokens: &mut Vec<Token>) {
    let mut value = String::new();
    let mut c = nth_char(chars, *curr);

    while *curr < chars.len() && c >= '0' && c <= '7' {
        value.push(c);
        *curr += 1;
        c = nth_char(chars, *curr);
    }

    tokens.push(Token {
        token_type: TokenType::OctLiteral,
        value,
    });
}

fn decimal_number_literal(curr: &mut usize, chars: &[char], tokens: &mut Vec<Token>) {
    let mut value = String::new();
    let mut c = nth_char(chars, *curr);

    let mut dot_count: usize = 0;

    while *curr < chars.len() && (is_digit(c) || c == '.') {
        if c == '.' {
            dot_count += 1;
        }
        value.push(c);
        *curr += 1;
        c = nth_char(chars, *curr);
    }

    if dot_count > 1 {
        tokens.push(Token {
            token_type: TokenType::Unknown,
            value,
        });
    } else {
        tokens.push(Token {
            token_type: TokenType::Number,
            value,
        });
    }
}

pub fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

pub fn is_hex_digit(c: char) -> bool {
    match c {
        '0'..='9' | 'a'..='f' | 'A'..='F' => true,
        _ => false,
    }
}
