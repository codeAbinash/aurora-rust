use crate::tokenizer::{nth_char, Token, TokenType};

const KEYWORDS: [&str; 32] = [
    "auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else",
    "enum", "extern", "float", "for", "goto", "if", "int", "long", "register", "return", "short",
    "signed", "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned", "void",
    "volatile", "while",
];

const DEFINED: [&str; 7] = ["NULL", "size_t", "true", "false", "bool", "FILE", "EOF"];

pub fn name_or_keyword(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    let mut c = nth_char(file, *curr);

    while *curr < file.len() && is_alphanumeric(c) {
        value.push(c);
        *curr += 1;
        c = nth_char(file, *curr);
    }

    if KEYWORDS.contains(&value.as_str()) {
        tokens.push(Token {
            token_type: TokenType::Keyword,
            value,
        });
        return;
    }
    // If the next character is a '(', then it's a function
    else if c == '(' {
        tokens.push(Token {
            token_type: TokenType::Function,
            value,
        });
        return;
    }
    // If it is a defined value
    else if DEFINED.contains(&value.as_str()) {
        tokens.push(Token {
            token_type: TokenType::Defined,
            value,
        });
        return;
    }

    tokens.push(Token {
        token_type: TokenType::Name,
        value,
    });
}

pub fn is_alphanumeric(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
        _ => false,
    }
}
