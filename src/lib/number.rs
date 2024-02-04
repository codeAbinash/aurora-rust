use crate::tokenizer::{nth_char, Token, TokenType};

pub fn number_literal(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let c = nth_char(file, *curr);
    let next = nth_char(file, *curr + 1);

    if c == '0' {
        // Check if it is a HexLiteral
        if next == 'x' || next == 'X' {
            push_hex_prefix(curr, file, tokens);
            hex_literal(curr, file, tokens);
            return;
        }

        // Check if it is a BinLiteral
        if next == 'b' || next == 'B' {
            push_bin_prefix(curr, file, tokens);
            bin_literal(curr, file, tokens);
            return;
        }

        // Decimal Number Literal
        if next == '.' {
            decimal_number_literal(curr, file, tokens);
            return;
        }

        // Check if it is an OctLiteral
        if next >= '0' && next <= '7' {
            push_oct_prefix(curr, file, tokens);
            oct_literal(curr, file, tokens);
            return;
        }

        decimal_number_literal(curr, file, tokens);
    } else {
        decimal_number_literal(curr, file, tokens);
    }
}

pub fn push_hex_prefix(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let c = nth_char(file, *curr);
    let next = nth_char(file, *curr + 1);
    tokens.push(Token {
        token_type: TokenType::HexPrefix,
        value: format!("{}{}", c, next),
    });
    *curr += 2;
}

pub fn push_bin_prefix(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let c = nth_char(file, *curr);
    let next = nth_char(file, *curr + 1);
    tokens.push(Token {
        token_type: TokenType::BinPrefix,
        value: format!("{}{}", c, next),
    });
    *curr += 2;
}

pub fn push_oct_prefix(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let c = nth_char(file, *curr);
    let next = nth_char(file, *curr + 1);
    tokens.push(Token {
        token_type: TokenType::OctPrefix,
        value: format!("{}{}", c, next),
    });
    *curr += 2;
}

fn hex_literal(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    let mut c = nth_char(file, *curr);

    while *curr < file.len() && is_hex_digit(c) {
        value.push(c);
        *curr += 1;
        c = nth_char(file, *curr);
    }

    tokens.push(Token {
        token_type: TokenType::HexLiteral,
        value,
    });
}

fn bin_literal(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    let mut c = nth_char(file, *curr);

    while *curr < file.len() && (c == '0' || c == '1') {
        value.push(c);
        *curr += 1;
        c = nth_char(file, *curr);
    }

    tokens.push(Token {
        token_type: TokenType::BinLiteral,
        value,
    });
}

fn oct_literal(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    let mut c = nth_char(file, *curr);

    while *curr < file.len() && c >= '0' && c <= '7' {
        value.push(c);
        *curr += 1;
        c = nth_char(file, *curr);
    }

    tokens.push(Token {
        token_type: TokenType::OctLiteral,
        value,
    });
}

fn decimal_number_literal(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    let mut c = nth_char(file, *curr);

    let mut dot_count: usize = 0;

    while *curr < file.len() && (is_digit(c) || c == '.') {
        if c == '.' {
            dot_count += 1;
        }
        value.push(c);
        *curr += 1;
        c = nth_char(file, *curr);
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
