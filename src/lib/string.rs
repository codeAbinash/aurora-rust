use crate::{
    escape::escape_sequence,
    tokenizer::{nth_char, Token, TokenType},
};

pub fn character_literal(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    value.push('\'');
    *curr += 1;
    let mut c = nth_char(file, *curr);

    while *curr < file.len() && c != '\'' {
        // Escape sequence
        if c == '\\' {
            tokens.push(Token {
                token_type: TokenType::String,
                value: value,
            });
            escape_sequence(curr, file, tokens);
            value = String::new(); // Reset value
            c = nth_char(file, *curr);
            continue;
        }

        value.push(c);
        *curr += 1;
        c = nth_char(file, *curr);
    }

    value.push('\'');
    *curr += 1;
    tokens.push(Token {
        token_type: TokenType::String,
        value,
    });
}

pub fn string_literal(curr: &mut usize, file: &String, tokens: &mut Vec<Token>) {
    let mut value = String::new();
    value.push('"');
    *curr += 1;
    let mut c = nth_char(file, *curr);

    while *curr < file.len() && c != '"' {
        if c == '\\' {
            tokens.push(Token {
                token_type: TokenType::String,
                value,
            });
            escape_sequence(curr, file, tokens);
            value = String::new(); // Reset value
            c = nth_char(file, *curr);
            continue;
        }

        // if c == '%' {
        //     *curr += 1;
        //     c = nth_char(file, *curr);

        //     let mut format_specifier = String::new();
        //     format_specifier.push('%');

        //     tokens.push(Token {
        //         token_type: TokenType::String,
        //         value,
        //     }); // Push the string literal
        //     value = String::new(); // Reset value

        //     // let mut dots = 0;

        //     while is_format_specifier_width(c) {
        //         // if c == '.' {
        //         //     dots += 1;
        //         // }
        //         format_specifier.push(c);
        //         *curr += 1;
        //         c = nth_char(file, *curr);
        //     }

        //     if c == 'd' {
        //         format_specifier.push(c);
        //         *curr += 1;
        //         c = nth_char(file, *curr);
        //     }

        //     tokens.push(Token {
        //         token_type: TokenType::FormatSpecifier,
        //         value: format_specifier,
        //     });

        // if dots > 1 {
        //     value.push_str(&format_specifier.to_string());
        //     tokens.push(Token {
        //         token_type: TokenType::String,
        //         value,
        //     });
        //     value = String::new(); // Reset value
        //     continue;
        // }

        // if dots >= 1 {
        //     tokens.push(Token {
        //         token_type: TokenType::FormatSpecifier,
        //         value: format_specifier,
        //     });
        //     value = String::new(); // Reset value
        //     continue;
        // }

        // let c1: char = nth_char(file, *curr);
        // let c2: char = nth_char(file, *curr + 1);
        // let c3: char = nth_char(file, *curr + 2);

        // match c1 {
        //     'l' => match c2 {
        //         'l' => {}
        //         _ => {}
        //     },
        //     _ => {}
        // }

        //     continue;
        // }

        value.push(c);
        *curr += 1;
        c = nth_char(file, *curr);
    }

    value.push('"');
    *curr += 1;
    tokens.push(Token {
        token_type: TokenType::String,
        value,
    });
}

// fn is_format_specifier_width(c: char) -> bool {
//     match c {
//         '0'..='9' => true,
//         '.' => true,
//         _ => false,
//     }
// }
