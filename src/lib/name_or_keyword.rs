use crate::tokenizer::{nth_byte, Token, TokenType};

pub fn name_or_keyword<'src>(
    curr: &mut usize,
    file: &'src str,
    bytes: &[u8],
    tokens: &mut Vec<Token<'src>>,
) {
    let start = *curr;
    while *curr < bytes.len() && is_alphanumeric(bytes[*curr]) {
        *curr += 1;
    }

    let val = &file[start..*curr];
    let t = match val {
        "auto" | "break" | "case" | "char" | "const" | "continue" | "default" | "do" | "double"
        | "else" | "enum" | "extern" | "float" | "for" | "goto" | "if" | "int" | "long"
        | "register" | "return" | "short" | "signed" | "sizeof" | "static" | "struct"
        | "switch" | "typedef" | "union" | "unsigned" | "void" | "volatile" | "while" => {
            TokenType::Keyword
        }

        "NULL" | "size_t" | "true" | "false" | "bool" | "FILE" | "EOF" => TokenType::Defined,

        _ if nth_byte(bytes, *curr) == b'(' => TokenType::Function,
        _ => TokenType::Name,
    };

    tokens.push(Token::new(t, val));
}

#[inline(always)]
pub fn is_alphanumeric(c: u8) -> bool {
    matches!(c, b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'0'..=b'9')
}
