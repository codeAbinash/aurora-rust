use std::io::{self, BufWriter, Write};

use colored::{ColoredString, Colorize};

use crate::tokenizer::{Token, TokenType};

// One Dark Pro theme colours.
const VIOLET: (u8, u8, u8) = (198, 120, 221);
const GREEN: (u8, u8, u8) = (152, 195, 121);
const GRAY: (u8, u8, u8) = (92, 99, 112);
const BLUE_GREEN: (u8, u8, u8) = (86, 182, 194);
const YELLOW: (u8, u8, u8) = (209, 154, 102);
const RED: (u8, u8, u8) = (238, 88, 111);
const BLUE: (u8, u8, u8) = (97, 175, 239);

pub fn highlighter(tokens: &[Token<'_>]) {
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    for token in tokens {
        let _ = write!(out, "{}", colorize(token));
    }

    let _ = out.flush();
}

fn colorize(token: &Token<'_>) -> ColoredString {
    let v = token.value;
    match token.token_type {
        TokenType::Operator
        | TokenType::Hash
        | TokenType::OpenCurly
        | TokenType::CloseCurly
        | TokenType::OpenParen
        | TokenType::CloseParen
        | TokenType::OpenSquare
        | TokenType::CloseSquare
        | TokenType::Colon
        | TokenType::Comma
        | TokenType::Semicolon => v.truecolor(VIOLET.0, VIOLET.1, VIOLET.2).bold(),

        TokenType::Preprocessor => v.truecolor(VIOLET.0, VIOLET.1, VIOLET.2).bold(),
        TokenType::HeaderFile => v.truecolor(GREEN.0, GREEN.1, GREEN.2).bold().underline(),
        TokenType::Keyword => v.truecolor(VIOLET.0, VIOLET.1, VIOLET.2).bold(),
        TokenType::Defined => v.truecolor(YELLOW.0, YELLOW.1, YELLOW.2).bold(),
        TokenType::Function => v.truecolor(BLUE.0, BLUE.1, BLUE.2).bold(),
        TokenType::Name => v.truecolor(RED.0, RED.1, RED.2).bold(),
        TokenType::String => v.truecolor(GREEN.0, GREEN.1, GREEN.2).bold(),
        TokenType::Escape => v.truecolor(BLUE_GREEN.0, BLUE_GREEN.1, BLUE_GREEN.2).bold(),
        TokenType::FormatSpecifier => v.truecolor(YELLOW.0, YELLOW.1, YELLOW.2).bold(),
        TokenType::Number => v.truecolor(YELLOW.0, YELLOW.1, YELLOW.2).bold(),
        TokenType::HexPrefix => v.truecolor(YELLOW.2, YELLOW.0, YELLOW.1).bold(),
        TokenType::HexLiteral => v.truecolor(YELLOW.2, YELLOW.1, YELLOW.2).bold(),
        TokenType::BinPrefix => v.truecolor(YELLOW.2, YELLOW.0, YELLOW.1).bold(),
        TokenType::BinLiteral => v.truecolor(YELLOW.2, YELLOW.1, YELLOW.2).bold(),
        TokenType::OctPrefix => v.truecolor(YELLOW.2, YELLOW.0, YELLOW.1).bold(),
        TokenType::OctLiteral => v.truecolor(YELLOW.2, YELLOW.1, YELLOW.2).bold(),
        TokenType::Comment => v.truecolor(GRAY.0, GRAY.1, GRAY.2).italic(),
        TokenType::Whitespace | TokenType::Unknown => v.normal(),
    }
}
