use std::{
    iter::{Enumerate, Peekable},
    str::Bytes,
};

struct AddressInstruction {
    address: u16,
}

enum Mode {
    A,
    M,
}

struct Destination(u8);

struct Computation(u8);

struct Comparison(u8);

struct ComputationInstruction {
    mode: Mode,
    destination: Destination,
    computation: Computation,
    comparison: Comparison,
}

#[derive(Debug)]
enum Token<'a> {
    // Symbols
    AtSymbol,
    LeftParenthesis,
    RightParenthesis,
    Equal,
    Semicolon,
    // Operators
    Plus,
    Minus,
    Not,
    Or,
    And,
    // Pre-defined Symbols
    R(u8),
    SP,
    LCL,
    ARG,
    THIS,
    THAT,
    SCREEN,
    KBD,
    // Other stuff
    Number(u16),
    Identifier(&'a str),
}

impl<'a> From<&'a [u8]> for Token<'a> {
    fn from(value: &'a [u8]) -> Self {
        match value {
            [b'R', digit @ b'0'..=b'9'] => Token::R(atoi(*digit)),
            [b'R', b'1', digit @ b'0'..=b'5'] => Token::R(10 + atoi(*digit)),
            b"SP" => Token::SP,
            b"LCL" => Token::LCL,
            b"ARG" => Token::ARG,
            b"THIS" => Token::THIS,
            b"THAT" => Token::THAT,
            b"SCREEN" => Token::SCREEN,
            b"KBD" => Token::KBD,
            _ => {
                // SAFETY: We converted this from &str earlier.
                let name = unsafe { std::str::from_utf8_unchecked(value) };
                Token::Identifier(name)
            }
        }
    }
}

struct Lexer<'a> {
    input: &'a [u8],
    file_iter: Peekable<Enumerate<Bytes<'a>>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (pos, byte) = self.skip_comments_and_whitespace()?;

        let token = match byte {
            // These two checks are here because we don't want to treat the A, D, M registers
            // as Identifiers. We instead would like to treat them as a special token. For
            // instance, we want @AD to refer to the symbol 'AD', not the A and D registers.
            b if b.is_ascii_digit() => self.get_number(pos),
            b if is_nondigit_identifier_character(b) => self.get_identifier(pos),
            b'@' => Token::AtSymbol,
            b'(' => Token::LeftParenthesis,
            b')' => Token::RightParenthesis,
            b'=' => Token::Equal,
            b';' => Token::Semicolon,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => Token::Not,
            b'|' => Token::Or,
            b'&' => Token::And,
            _ => panic!("Unexpected character: {}", byte.escape_ascii().to_string()),
        };
        Some(token)
    }
}

impl<'a> Lexer<'a> {
    fn new(file_contents: &'a str) -> Self {
        Self {
            input: file_contents.as_bytes(),
            file_iter: file_contents.bytes().enumerate().peekable(),
        }
    }

    // Returns Some(first byte) that isn't part of a comment or whitespace, if there is one.
    // Otherwise, returns None.
    fn skip_comments_and_whitespace(&mut self) -> Option<(usize, u8)> {
        loop {
            let (pos, byte) = self.file_iter.next()?;
            if byte == b'/' {
                if let Some((_, b'/')) = self.file_iter.next() {
                    self.file_iter.find(|&(_, b)| b == b'\n');
                    continue;
                } else {
                    panic!("Expected another '/'.")
                }
            } else if byte.is_ascii_whitespace() {
                continue;
            } else {
                return Some((pos, byte));
            }
        }
    }

    fn get_identifier(&mut self, start: usize) -> Token<'a> {
        let mut end = start;
        while let Some((pos, _)) = self
            .file_iter
            .next_if(|&(_, b)| b.is_ascii_digit() | is_nondigit_identifier_character(b))
        {
            end = pos
        }
        Token::from(&self.input[start..=end])
    }

    fn get_number(&mut self, start: usize) -> Token<'a> {
        let mut end = start;
        while let Some((pos, _)) = self.file_iter.next_if(|&(_, b)| b.is_ascii_digit()) {
            end = pos
        }
        // SAFETY: We converted this &[u8] from a str in the beginning AND we know that all the
        // values are digits.
        let number = unsafe {
            std::str::from_utf8_unchecked(&self.input[start..=end])
                .parse()
                .unwrap_unchecked()
        };
        Token::Number(number)
    }
}

fn is_nondigit_identifier_character(byte: u8) -> bool {
    matches!(byte, b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'.' | b'$' | b':')
}

#[inline]
fn atoi(byte: u8) -> u8 {
    byte - b'0'
}

fn main() {
    let file_contents = std::fs::read_to_string("../max/Max.asm").expect("Path not found.");
    let lexer = Lexer::new(&file_contents);
    let lexed_file: Vec<Token<'_>> = lexer.collect();
    print!("{:#?}", lexed_file)
}
