use crate::tokenizer::{nth_char, Token, TokenType};

pub fn single_line_comment(curr: &mut usize, code: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    value.push('/');
    value.push('/');
    *curr += 1;
    let mut c = nth_char(code, *curr);
    while *curr < code.len() && c != '\n' {
        value.push(c);
        *curr += 1;
        c = nth_char(code, *curr);
    }
    tokens.push(Token {
        token_type: TokenType::Comment,
        value,
    });
}

pub fn multi_line_comment(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    value.push('/');
    value.push('*');
    *curr += 1;
    while *curr < file.len() {
        let c = nth_char(file, *curr);
        value.push(c);
        *curr += 1;
        if c == '*' {
            let c = nth_char(file, *curr);
            value.push(c);
            *curr += 1;
            if c == '/' {
                break;
            }
        }
    }
    tokens.push(Token {
        token_type: TokenType::Comment,
        value,
    });
}
