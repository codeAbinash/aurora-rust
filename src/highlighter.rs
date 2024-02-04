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
            TokenType::Preprocessor => print!("{}", Color::preprocessor(&token.value)),
            TokenType::Defined => print!("{}", Color::defined(&token.value)),
            TokenType::Keyword => print!("{}", Color::keyword(&token.value)),
            TokenType::Name => print!("{}", Color::name(&token.value)),
            TokenType::String => print!("{}", Color::string(&token.value)),
            // TokenType::Quote => print!("{}", token.value),
            TokenType::FormatSpecifier => print!("{}", Color::format_specifier(&token.value)),
            TokenType::Number => print!("{}", Color::number(&token.value)),
            TokenType::BinPrefix => print!("{}", Color::bin_prefix(&token.value)),
            TokenType::BinLiteral => print!("{}", Color::bin_literal(&token.value)),
            TokenType::HexPrefix => print!("{}", Color::hex_prefix(&token.value)),
            TokenType::HexLiteral => print!("{}", Color::hex_literal(&token.value)),
            TokenType::OctPrefix => print!("{}", Color::oct_prefix(&token.value)),
            TokenType::OctLiteral => print!("{}", Color::oct_literal(&token.value)),
            TokenType::Function => print!("{}", Color::function(&token.value)),
            TokenType::Escape => print!("{}", Color::escape(&token.value)),
            TokenType::Unknown => print!("{}", token.value),
        }
    }
}

// Using One Dark Pro Theme

struct Color {}

const VIOLET: (u8, u8, u8) = (198, 120, 221);
const GREEN: (u8, u8, u8) = (152, 195, 121);
const GRAY: (u8, u8, u8) = (92, 99, 112);
const BLUE_GREEN: (u8, u8, u8) = (86, 182, 194);
const YELLOW: (u8, u8, u8) = (209, 154, 102);
const RED: (u8, u8, u8) = (238, 88, 111);
const BLUE: (u8, u8, u8) = (97, 175, 239);

impl Color {
    fn operator(s: &String) -> ColoredString {
        return s.truecolor(VIOLET.0, VIOLET.1, VIOLET.2).bold();
    }
    fn comment(s: &String) -> ColoredString {
        return s.truecolor(GRAY.0, GRAY.1, GRAY.2).italic();
    }
    fn header_file(s: &String) -> ColoredString {
        return s.truecolor(GREEN.0, GREEN.1, GREEN.2).bold().underline();
    }
    fn preprocessor(s: &String) -> ColoredString {
        return s.truecolor(VIOLET.0, VIOLET.1, VIOLET.2).bold();
    }
    fn string(s: &String) -> ColoredString {
        return s.truecolor(GREEN.0, GREEN.1, GREEN.2).bold();
    }
    fn escape(s: &String) -> ColoredString {
        return s.truecolor(BLUE_GREEN.0, BLUE_GREEN.1, BLUE_GREEN.2).bold();
    }
    fn format_specifier(s: &String) -> ColoredString {
        return s.truecolor(YELLOW.0, YELLOW.1, YELLOW.2).bold();
    }
    fn number(s: &String) -> ColoredString {
        return s.truecolor(YELLOW.0, YELLOW.1, YELLOW.2).bold();
    }
    fn hex_prefix(s: &String) -> ColoredString {
        return s.truecolor(YELLOW.2, YELLOW.0, YELLOW.1).bold();
    }
    fn hex_literal(s: &String) -> ColoredString {
        return s.truecolor(YELLOW.2, YELLOW.1, YELLOW.2).bold();
    }
    fn bin_prefix(s: &String) -> ColoredString {
        return s.truecolor(YELLOW.2, YELLOW.0, YELLOW.1).bold();
    }
    fn bin_literal(s: &String) -> ColoredString {
        return s.truecolor(YELLOW.2, YELLOW.1, YELLOW.2).bold();
    }
    fn oct_prefix(s: &String) -> ColoredString {
        return s.truecolor(YELLOW.2, YELLOW.0, YELLOW.1).bold();
    }
    fn oct_literal(s: &String) -> ColoredString {
        return s.truecolor(YELLOW.2, YELLOW.1, YELLOW.2).bold();
    }
    fn keyword(s: &String) -> ColoredString {
        return s.truecolor(VIOLET.0, VIOLET.1, VIOLET.2).bold();
    }
    fn defined(s: &String) -> ColoredString {
        return s.truecolor(YELLOW.0, YELLOW.1, YELLOW.2).bold();
    }
    fn name(s: &String) -> ColoredString {
        return s.truecolor(RED.0, RED.1, RED.2).bold();
    }
    fn function(s: &String) -> ColoredString {
        return s.truecolor(BLUE.0, BLUE.1, BLUE.2).bold();
    }
}
