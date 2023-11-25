use crate::{
    comments::{multi_line_comment, single_line_comment},
    highlighter,
    preprocessor::preprocessor_literal, string::character_literal,
};

pub enum TokenType {
    // CommentStart,
    // CommentEnd,
    Comment,
    Preprocessor,
    Defined,
    HeaderFile,
    Keyword,
    Name,
    String,
    Quote,
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
    tokenize(file, &mut tokens);
    highlighter(&tokens);
}

pub fn tokenize(file: &String, tokens: &mut Vec<Token>) {
    let mut curr = 0;
    let len = file.len();

    while curr < len {
        let mut c = nth_char(file, curr);

        if is_whitespace(c) {
            // Whitespace
            let mut value = String::new();
            while is_whitespace(c) {
                value.push(c);
                curr += 1;
                c = nth_char(file, curr);
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
            c = nth_char(file, curr);
            if c == '/' {
                single_line_comment(&mut curr, file, tokens);
            } else if c == '*' {
                multi_line_comment(&mut curr, file, tokens);
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
            character_literal(&mut curr, file, tokens);
            continue;
        }


        // Preprocessor
        if c == '#' && (curr == 0 || nth_char(file, curr - 1) == '\n') {
            preprocessor_literal(&mut curr, file, tokens);
            continue;
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

pub fn nth_char(s: &String, n: usize) -> char {
    let c = s.chars().nth(n);
    match c {
        Some(c) => c,
        None => '\0',
    }
}
