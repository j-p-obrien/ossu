use std::{
    cell::Cell,
    collections::HashMap,
    iter::{Enumerate, Peekable},
    rc::Rc,
    str::Bytes,
};

type Address = u16;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Identifier<'a>(&'a str);

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
    Identifier(Identifier<'a>),
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
                Token::Identifier(Identifier(name))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Lexer<'a> {
    file_contents: &'a [u8],
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
            file_contents: file_contents.as_bytes(),
            file_iter: file_contents.bytes().enumerate().peekable(),
        }
    }

    // Returns Some(byte) where byte is the first byte that isn't part of a comment or whitespace,
    // if there is one. Otherwise, returns None.
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
        Token::from(&self.file_contents[start..=end])
    }

    fn get_number(&mut self, start: usize) -> Token<'a> {
        let mut end = start;
        while let Some((pos, _)) = self.file_iter.next_if(|&(_, b)| b.is_ascii_digit()) {
            end = pos
        }
        // SAFETY: We converted this &[u8] from a str in the beginning AND we know that all the
        // values are digits.
        let number = unsafe {
            std::str::from_utf8_unchecked(&self.file_contents[start..=end])
                .parse()
                .unwrap_unchecked()
        };
        Token::Number(number)
    }
}

struct LabelTable<'a>(HashMap<Identifier<'a>, AddressInstruction>);

impl<'a> LabelTable<'a> {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

struct SymbolTable<'a> {
    table: HashMap<Identifier<'a>, Address>,
    next_address: Address,
}

impl<'a> SymbolTable<'a> {
    fn new() -> Self {
        let table = HashMap::new();
        Self {
            table,
            next_address: 16,
        }
    }
}

// !!! Symbols are lower priority than labels.
struct Parser<'a> {
    lexer: Lexer<'a>,
    instructions_parsed: Address,
    labels: LabelTable<'a>,
    symbols: SymbolTable<'a>,
}

#[derive(Debug)]
enum AddressInstruction {
    Definite(Address),
    // Sometimes we come across a label in an '@' command before the label itself is defined.
    // Thus, we use a shared reference so we can update the value without a second pass over
    // the file.
    Indefinite(Rc<Cell<Option<Address>>>),
}

#[derive(Debug)]
enum Mode {
    A,
    M,
}

#[derive(Debug)]
struct Destination(Address);

#[derive(Debug)]
struct Computation(Address);

#[derive(Debug)]
struct Jump(Address);

#[derive(Debug)]
struct ComputationInstruction {
    mode: Mode,
    destination: Destination,
    computation: Computation,
    comparison: Jump,
}

#[derive(Debug)]
enum Instruction {
    Computation(ComputationInstruction),
    Address(AddressInstruction),
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            instructions_parsed: 0,
            labels: LabelTable::new(),
            symbols: SymbolTable::new(),
        }
    }

    fn add_label(&mut self, label: Identifier<'a>) {
        todo!()
    }

    fn label_instruction(&mut self) {
        let label_token = self.lexer.next().expect("Didn't expect EOF.");
        if let Token::Identifier(label) = label_token {
            self.add_label(label);
        } else {
            panic!(
                "Expected a label at instruction: {}.",
                self.instructions_parsed
            )
        }
    }

    fn address_instruction(&mut self) -> AddressInstruction {
        let address = self.lexer.next().expect("Didn't expect EOF.");
        match address {
            Token::R(num) => todo!(),
            Token::SP => todo!(),
            Token::LCL => todo!(),
            Token::ARG => todo!(),
            Token::THIS => todo!(),
            Token::THAT => todo!(),
            Token::SCREEN => todo!(),
            Token::KBD => todo!(),
            Token::Number(_) => todo!(),
            Token::Identifier(identifier) => todo!(),
            _ => panic!(),
        }
        todo!()
    }

    fn computation_instruction(&mut self) -> ComputationInstruction {
        todo!()
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token = self.lexer.next()?;
        // This allows multiple label instructions in a row, which I think is technically allowed?
        // I don't see anything in the spec forbidding it, so I suppose we will allow it.
        while let Token::LeftParenthesis = token {
            self.label_instruction();
            token = self.lexer.next()?;
        }
        Some(match token {
            Token::AtSymbol => Instruction::Address(self.address_instruction()),
            Token::Number(_) => Instruction::Computation(self.computation_instruction()),
            Token::Identifier(_) => Instruction::Computation(self.computation_instruction()),
            _ => todo!(),
        })
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
    let parser = Parser::new(lexer.clone());
    let lexed_file: Vec<Token<'_>> = lexer.collect();
    let parsed_file: Vec<Instruction> = parser.collect();
    print!("{:#?}", lexed_file);
    print!("{:#?}", parsed_file)
}
