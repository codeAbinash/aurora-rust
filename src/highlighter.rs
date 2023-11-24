use colored::{ColoredString, Colorize};

use crate::tokenizer::{Token, TokenType};

pub fn highlighter(tokens: &Vec<Token>) {
    for token in tokens {
        match token.token_type {
            TokenType::Whitespace => print!("{}", token.value),
            TokenType::Comment => print!("{}", Color::comment(&token.value)),
            TokenType::Operator => print!("{}", Color::operator(&token.value)),
            TokenType::Hash => print!("{}", Color::operator(&token.value)),
            TokenType::OpenCurly => print!("{}", Color::operator(&token.value)),
            TokenType::CloseCurly => print!("{}", Color::operator(&token.value)),
            TokenType::OpenParen => print!("{}", Color::operator(&token.value)),
            TokenType::CloseParen => print!("{}", Color::operator(&token.value)),
            TokenType::OpenSquare => print!("{}", Color::operator(&token.value)),
            TokenType::CloseSquare => print!("{}", Color::operator(&token.value)),
            TokenType::Colon => print!("{}", Color::operator(&token.value)),
            TokenType::Comma => print!("{}", Color::operator(&token.value)),
            TokenType::Semicolon => print!("{}", Color::operator(&token.value)),
            TokenType::HeaderFile => print!("{}", Color::header_file(&token.value)),
            TokenType::Preprocessor => print!("{}", token.value),
            TokenType::Defined => print!("{}", token.value),
            TokenType::Keyword => print!("{}", token.value),
            TokenType::Name => print!("{}", token.value),
            TokenType::String => print!("{}", token.value),
            TokenType::Quote => print!("{}", token.value),
            TokenType::FormatSpecifier => print!("{}", token.value),
            TokenType::Number => print!("{}", token.value),
            TokenType::BinPrefix => print!("{}", token.value),
            TokenType::OctPrefix => print!("{}", token.value),
            TokenType::HexPrefix => print!("{}", token.value),
            TokenType::BinLiteral => print!("{}", token.value),
            TokenType::OctLiteral => print!("{}", token.value),
            TokenType::HexLiteral => print!("{}", token.value),
            TokenType::Function => print!("{}", token.value),
            TokenType::Escape => print!("{}", token.value),
            TokenType::Unknown => print!("{}", token.value),
        }
    }
}

// Using One Dark Pro Theme

struct Color {}

impl Color {
    fn operator(s: &String) -> ColoredString {
        return s.truecolor(198, 120, 221);
    }

    fn comment(s: &String) -> ColoredString {
        return s.truecolor(92, 99, 112).italic();
    }
    fn header_file(s: &String) -> ColoredString {
        return s.truecolor(199, 146, 234);
    }
}
