//! The BITS decoder utilities

const VERSION_BITS: usize = 3;
const PACKET_TYPE_BITS: usize = 3;
const LENGTH_TYPE_BITS: usize = 1;
const LIT_FRAGMENT_BITS: usize = 5;
const LEN_BITWISE_BITS: usize = 15;
const LEN_SUBPACKETS_BITS: usize = 11;

const LIT_PACKET_TYPE: u8 = 4;

pub fn hexes_to_bits<I: IntoIterator<Item = u8>>(it: I) -> impl Iterator<Item = u8> {
    it.into_iter()
        .flat_map(|hex| [hex >> 3 & 0x1, hex >> 2 & 0x1, hex >> 1 & 0x1, hex & 0x1])
}

#[derive(Clone, Copy, Debug)]
pub enum Token {
    Version(u8),
    PacketType(u8),
    LengthBitwise(u16),
    LengthSubpackets(u16),
    LiteralFragment(bool, u8),
}

#[derive(Clone, Copy, Debug)]
enum Next {
    Version,
    PacketType,
    Length,
    LitFragment,
    None,
}

#[derive(Clone, Copy, Debug)]
enum Remaining {
    Bits(u16),
    Packets(u16),
}

// requires a bit-by-bit iterator
pub struct Lexer<I> {
    it: I,
    state: Next,
    stack: Vec<Remaining>,
}

pub struct Decoder {}

impl Token {
    fn unwrap_type_id(self) -> u8 {
        match self {
            Self::PacketType(v) => v,
            _ => panic!("Failed to unwrap type id"),
        }
    }

    fn unwrap_lit_fragment(self) -> (bool, u8) {
        match self {
            Self::LiteralFragment(is_cont, value) => (is_cont, value),
            _ => panic!("Failed to unwrap literal fragment"),
        }
    }

    fn unwrap_len_subpackets(self) -> u16 {
        match self {
            Self::LengthSubpackets(v) => v,
            _ => panic!("Failed to unwrap length subpackets"),
        }
    }

    fn unwrap_len_bitwise(self) -> u16 {
        match self {
            Self::LengthBitwise(v) => v,
            _ => panic!("Failed to unwrap length bitwise"),
        }
    }
}

impl<I: Iterator<Item = u8>> Lexer<I> {
    pub fn from_bits<T: IntoIterator<IntoIter = I>>(bits: T) -> Self {
        Self {
            it: bits.into_iter(),
            state: Next::Version,
            stack: vec![Remaining::Packets(1)],
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        let (result, next_state) = match self.state {
            Next::Version => (self.read_version(), Next::PacketType),
            Next::PacketType => {
                let token = self.read_packet_type();
                let type_id = token.unwrap_type_id();

                let next = match type_id {
                    LIT_PACKET_TYPE => Next::LitFragment,
                    _ => Next::Length,
                };

                (token, next)
            }
            Next::Length => {
                let length_type = self.read_bits(LENGTH_TYPE_BITS);

                let token = if length_type == 1 {
                    let t = self.read_length_subpackets();
                    let len = t.unwrap_len_subpackets();

                    self.stack.push(Remaining::Packets(len));

                    t
                } else {
                    let t = self.read_length_bitwise();
                    let len = t.unwrap_len_bitwise();

                    self.stack.push(Remaining::Bits(len));

                    t
                };

                (token, Next::Version)
            }
            Next::LitFragment => {
                let token = self.read_lit_fragment();
                let (is_cont, _) = token.unwrap_lit_fragment();

                let next = if is_cont {
                    Next::LitFragment
                } else {
                    self.try_collapse_stack();

                    if self.stack.is_empty() {
                        Next::None
                    } else {
                        Next::Version
                    }
                };

                (token, next)
            }
            Next::None => {
                return None;
            }
        };

        self.state = next_state;

        Some(result)
    }

    // read and return at most `n` bits (up to 16)
    fn read_bits(&mut self, n: usize) -> u16 {
        assert!(n <= 16, "Attempted to read more than 16 bits at once");

        for rem in &mut self.stack {
            if let Remaining::Bits(b) = rem {
                // *b -= n as u16;
                *b = b.checked_sub(n as u16).unwrap();
            }
        }

        self.it
            .by_ref()
            .take(n)
            .fold(0, |acc, bit| acc << 1 | (bit & 0x1) as u16)
    }

    fn read_version(&mut self) -> Token {
        let bits = self.read_bits(VERSION_BITS) as u8;

        Token::Version(bits)
    }

    fn read_packet_type(&mut self) -> Token {
        let bits = self.read_bits(PACKET_TYPE_BITS) as u8;

        Token::PacketType(bits)
    }

    fn read_lit_fragment(&mut self) -> Token {
        let bits = self.read_bits(LIT_FRAGMENT_BITS) as u8;

        let is_continuation = (bits & 0b10000) != 0;
        let value = bits & 0xf;

        Token::LiteralFragment(is_continuation, value)
    }

    fn read_length_bitwise(&mut self) -> Token {
        let bits = self.read_bits(LEN_BITWISE_BITS);

        Token::LengthBitwise(bits)
    }

    fn read_length_subpackets(&mut self) -> Token {
        let bits = self.read_bits(LEN_SUBPACKETS_BITS);

        Token::LengthSubpackets(bits)
    }

    fn try_collapse_stack(&mut self) {
        loop {
            match self.stack.last_mut() {
                Some(Remaining::Packets(1) | Remaining::Bits(0)) => {
                    self.stack.pop();
                }
                Some(Remaining::Packets(n)) => {
                    *n -= 1;
                    return;
                }
                _ => {
                    return;
                }
            }
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for Lexer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
