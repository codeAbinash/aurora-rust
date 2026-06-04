use crate::tokenizer::{nth_char, Token, TokenType};

pub fn preprocessor_literal(curr: &mut usize, chars: &[char], tokens: &mut Vec<Token>) {
    let mut value = String::new();
    value.push('#');
    *curr += 1;
    let mut c = nth_char(chars, *curr);

    while *curr < chars.len()
        && (c != ' ' && c != '\n' && c != '\t' && c != '\r' && c != '"' && c != '<')
    {
        value.push(c);
        *curr += 1;
        c = nth_char(chars, *curr);
    }

    tokens.push(Token {
        token_type: TokenType::Preprocessor,
        value,
    });

    // If the preprocessor is `#include` then it is a header file
    let value = tokens.last().unwrap().value.clone();
    if value == "#include" {
        header_file(curr, chars, tokens);
    }
}

fn header_file(curr: &mut usize, chars: &[char], tokens: &mut Vec<Token>) {
    let mut c = nth_char(chars, *curr);

    let mut value = String::new();
    // Ignore spaces
    while *curr < chars.len() && c != '"' && c != '<' {
        value.push(c);
        *curr += 1;
        c = nth_char(chars, *curr);
    }
    tokens.push(Token {
        token_type: TokenType::Whitespace,
        value,
    });

    value = String::new();
    
    if c == '"' {
        value.push('"');
        *curr += 1;
        c = nth_char(chars, *curr);

        while *curr < chars.len() && c != '"' {
            value.push(c);
            *curr += 1;
            c = nth_char(chars, *curr);
        }

        value.push('"');
        *curr += 1;
        tokens.push(Token {
            token_type: TokenType::HeaderFile,
            value,
        });
        return;
    }

    if c == '<' {
        value.push('<');
        *curr += 1;
        c = nth_char(chars, *curr);

        while *curr < chars.len() && c != '>' {
            value.push(c);
            *curr += 1;
            c = nth_char(chars, *curr);
        }

        value.push('>');
        *curr += 1;
        tokens.push(Token {
            token_type: TokenType::HeaderFile,
            value,
        });
    }
}
