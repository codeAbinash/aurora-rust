use std::time::Instant;

#[allow(unused_imports)]
use crate::{
    comments::{multi_line_comment, single_line_comment},
    highlighter,
    name_or_keyword::name_or_keyword,
    number::number_literal,
    preprocessor::preprocessor_literal,
    string::{character_literal, string_literal},
};

#[allow(dead_code)]
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

pub struct Token<'src> {
    pub token_type: TokenType,
    pub value: &'src str,
}

impl<'src> Token<'src> {
    #[inline(always)]
    pub fn new(token_type: TokenType, value: &'src str) -> Self {
        Self { token_type, value }
    }
}

pub fn engine(file: &str) {
    let mut tokens = Vec::with_capacity(file.len() / 4);
    let start = Instant::now();
    tokenize(file, &mut tokens);
    let elapsed = start.elapsed();

    println!("Token count: {}", tokens.len());
    println!("Tokenizing took {:?}", elapsed);
    // highlighter::highlighter(&tokens);
}

pub fn tokenize<'src>(file: &'src str, tokens: &mut Vec<Token<'src>>) {
    let bytes = file.as_bytes();
    let mut curr = 0;

    while curr < bytes.len() {
        let c = bytes[curr];

        match c {
            // Whitespace
            b' ' | b'\t' | b'\n' | b'\r' => {
                let start = curr;
                while curr < bytes.len() && matches!(bytes[curr], b' ' | b'\t' | b'\n' | b'\r') {
                    curr += 1;
                }
                tokens.push(Token::new(TokenType::Whitespace, &file[start..curr]));
            }

            // Comments and division
            b'/' => {
                curr += 1;
                if curr < bytes.len() && bytes[curr] == b'/' {
                    single_line_comment(&mut curr, file, bytes, tokens);
                } else if curr < bytes.len() && bytes[curr] == b'*' {
                    multi_line_comment(&mut curr, file, bytes, tokens);
                } else {
                    tokens.push(Token::new(TokenType::Operator, &file[curr - 1..curr]));
                }
            }

            // Literals
            b'\'' => character_literal(&mut curr, file, bytes, tokens),
            b'"' => string_literal(&mut curr, file, bytes, tokens),
            b'0'..=b'9' => number_literal(&mut curr, file, bytes, tokens),

            // Preprocessor or Hash
            b'#' => {
                if curr == 0 || bytes[curr - 1] == b'\n' {
                    preprocessor_literal(&mut curr, file, bytes, tokens);
                } else {
                    tokens.push(Token::new(TokenType::Hash, &file[curr..curr + 1]));
                    curr += 1;
                }
            }

            // Identifiers / Keywords
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => name_or_keyword(&mut curr, file, bytes, tokens),

            // Punctuation and Operators
            b'{' | b'}' | b'(' | b')' | b'[' | b']' | b':' | b',' | b';' | b'.' | b'+' | b'-'
            | b'*' | b'=' | b'&' | b'|' | b'^' | b'~' | b'!' | b'%' | b'<' | b'>' | b'?' => {
                let t = match c {
                    b'{' => TokenType::OpenCurly,
                    b'}' => TokenType::CloseCurly,
                    b'(' => TokenType::OpenParen,
                    b')' => TokenType::CloseParen,
                    b'[' => TokenType::OpenSquare,
                    b']' => TokenType::CloseSquare,
                    b':' => TokenType::Colon,
                    b',' => TokenType::Comma,
                    b';' => TokenType::Semicolon,
                    _ => TokenType::Operator,
                };
                tokens.push(Token::new(t, &file[curr..curr + 1]));
                curr += 1;
            }

            // Unknown
            _ => curr += 1,
        }
    }
}

#[inline(always)]
pub fn nth_byte(bytes: &[u8], n: usize) -> u8 {
    *bytes.get(n).unwrap_or(&0)
}
