use std::time::Instant;

use crate::{
    comments::{multi_line_comment, single_line_comment},
    highlighter,
    name_or_keyword::name_or_keyword,
    number::number_literal,
    preprocessor::preprocessor_literal,
    string::{character_literal, string_literal},
};

pub enum TokenType {
    // CommentStart,
    // CommentEnd,
    Defined,
    Comment,
    Preprocessor,
    HeaderFile,
    Keyword,
    Name,
    String,
    // Quote,
    FormatSpecifier,
    Number,
    BinPrefix,
    OctPrefix,
    HexPrefix,
    BinLiteral,
    OctLiteral,
    HexLiteral,
    Function,
    Operator,
    Escape,
    Comma,
    Semicolon,
    Colon,
    Hash,
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Whitespace,
    Unknown,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub fn engine(file: &String) {
    let mut tokens: Vec<Token> = Vec::new();
    let start = Instant::now();
    tokenize(file, &mut tokens);
    let end = Instant::now();
    println!("Token count: {}", tokens.len());
    highlighter(&tokens);
    println!("Tokenizing took {:?}", end - start);
}

pub fn tokenize(file: &String, tokens: &mut Vec<Token>) {
    let chars: Vec<char> = file.chars().collect();
    let len = chars.len();
    let mut curr = 0;

    while curr < len {
        let mut c = nth_char(&chars, curr);

        if is_whitespace(c) {
            // Whitespace
            let mut value = String::new();
            while is_whitespace(c) {
                value.push(c);
                curr += 1;
                c = nth_char(&chars, curr);
            }
            tokens.push(Token {
                token_type: TokenType::Whitespace,
                value,
            });
            continue;
        }

        // Comments
        if c == '/' {
            curr += 1;
            c = nth_char(&chars, curr);
            if c == '/' {
                single_line_comment(&mut curr, &chars, tokens);
            } else if c == '*' {
                multi_line_comment(&mut curr, &chars, tokens);
            } else {
                tokens.push(Token {
                    token_type: TokenType::Operator,
                    value: "/".to_string(),
                });
            }
            continue;
        }

        // Character Literal
        if c == '\'' {
            character_literal(&mut curr, &chars, tokens);
            continue;
        }
        // String Literal
        if c == '"' {
            string_literal(&mut curr, &chars, tokens);
            continue;
        }
        // Number Literal
        if c >= '0' && c <= '9' {
            number_literal(&mut curr, &chars, tokens);
            continue;
        }

        // Preprocessor
        if c == '#' && (curr == 0 || nth_char(&chars, curr - 1) == '\n') {
            preprocessor_literal(&mut curr, &chars, tokens);
            continue;
        }

        // Name or Keyword
        match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                name_or_keyword(&mut curr, &chars, tokens);
                continue;
            }
            _ => {}
        }

        match c {
            '#' => {
                tokens.push(Token {
                    token_type: TokenType::Hash,
                    value: "#".to_string(),
                });
            }
            '{' => {
                tokens.push(Token {
                    token_type: TokenType::OpenCurly,
                    value: "{".to_string(),
                });
            }
            '}' => {
                tokens.push(Token {
                    token_type: TokenType::CloseCurly,
                    value: "}".to_string(),
                });
            }
            '(' => {
                tokens.push(Token {
                    token_type: TokenType::OpenParen,
                    value: "(".to_string(),
                });
            }
            ')' => {
                tokens.push(Token {
                    token_type: TokenType::CloseParen,
                    value: ")".to_string(),
                });
            }
            '[' => {
                tokens.push(Token {
                    token_type: TokenType::OpenSquare,
                    value: "[".to_string(),
                });
            }
            ']' => {
                tokens.push(Token {
                    token_type: TokenType::CloseSquare,
                    value: "]".to_string(),
                });
            }
            '+' | '-' | '*' | '/' | '=' | '&' | '|' | '^' | '~' | '!' | '%' | '<' | '>' | '?' => {
                tokens.push(Token {
                    token_type: TokenType::Operator,
                    value: c.to_string(),
                });
            }
            ':' => {
                tokens.push(Token {
                    token_type: TokenType::Colon,
                    value: ":".to_string(),
                });
            }
            '.' => {
                tokens.push(Token {
                    token_type: TokenType::Operator,
                    value: ".".to_string(),
                });
            }
            ',' => {
                tokens.push(Token {
                    token_type: TokenType::Comma,
                    value: ",".to_string(),
                });
            }
            ';' => {
                tokens.push(Token {
                    token_type: TokenType::Semicolon,
                    value: ";".to_string(),
                });
            }

            _ => {}
        }

        curr += 1;
    }
}

pub fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

pub fn nth_char(chars: &[char], n: usize) -> char {
    if n < chars.len() {
        chars[n]
    } else {
        '\0'
    }
}
