//! The BITS decoder utilities.

const VERSION_BITS: usize = 3;
const PACKET_TYPE_BITS: usize = 3;
const LENGTH_TYPE_BITS: usize = 1;
const LIT_FRAGMENT_BITS: usize = 5;
const LEN_BITWISE_BITS: usize = 15;
const LEN_SUBPACKETS_BITS: usize = 11;

const SUM_PACKET_ID: u8 = 0;
const PRODUCT_PACKET_ID: u8 = 1;
const MIN_PACKET_ID: u8 = 2;
const MAX_PACKET_ID: u8 = 3;
const LIT_PACKET_ID: u8 = 4;
const GT_PACKET_ID: u8 = 5;
const LT_PACKET_ID: u8 = 6;
const EQ_PACKET_ID: u8 = 7;

/// Converts an iterable over hex bytes to an iterator of bits.
pub fn hexes_to_bits<I: IntoIterator<Item = u8>>(it: I) -> impl Iterator<Item = u8> {
    it.into_iter()
        .flat_map(|hex| [hex >> 3 & 0x1, hex >> 2 & 0x1, hex >> 1 & 0x1, hex & 0x1])
}

/// Represents various atomic parts of a BITS message.
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

// each packet in decoder stack
// keeps track of how many bits/subpackets are left to read
enum Packet {
    Sum(Option<Remaining>, Vec<u64>),
    Product(Option<Remaining>, Vec<u64>),
    Min(Option<Remaining>, Vec<u64>),
    Max(Option<Remaining>, Vec<u64>),
    Lit(u64),
    Gt(Option<Remaining>, Vec<u64>),
    Lt(Option<Remaining>, Vec<u64>),
    Eq(Option<Remaining>, Vec<u64>),
}

/// Transforms a bit-by-bit iterator into a stream of [`Token`]s.
pub struct Lexer<I> {
    it: I,
    state: Next,
    stack: Vec<Remaining>,
}

/// Fully decodes and evaluates the expression in a BITS message.
pub struct Decoder<I> {
    lexer: Lexer<I>,
    stack: Vec<Packet>,
}

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

impl Packet {
    fn remaining_mut(&mut self) -> Option<&mut Remaining> {
        use Packet::*;

        match self {
            Sum(rem, ..)
            | Product(rem, ..)
            | Min(rem, ..)
            | Max(rem, ..)
            | Gt(rem, ..)
            | Lt(rem, ..)
            | Eq(rem, ..) => rem.as_mut(),
            _ => None,
        }
    }

    fn remaining_as_mut(&mut self) -> &mut Option<Remaining> {
        use Packet::*;

        match self {
            Sum(result, ..)
            | Product(result, ..)
            | Min(result, ..)
            | Max(result, ..)
            | Gt(result, ..)
            | Lt(result, ..)
            | Eq(result, ..) => result,
            _ => panic!("Tried to obtain mut ref to remaining field where there was none"),
        }
    }
}

impl<I: Iterator<Item = u8>> Lexer<I> {
    /// Creates a new [`Lexer`] from a bit-by-bit iterable.
    pub fn from_bits<T: IntoIterator<IntoIter = I>>(bits: T) -> Self {
        Self {
            it: bits.into_iter(),
            state: Next::Version,
            stack: vec![Remaining::Packets(1)],
        }
    }

    fn next_token(&mut self) -> Option<(Token, usize)> {
        let (result, bits_read, next_state) = match self.state {
            Next::Version => (self.read_version(), VERSION_BITS, Next::PacketType),
            Next::PacketType => {
                let token = self.read_packet_type();
                let type_id = token.unwrap_type_id();

                let next = match type_id {
                    LIT_PACKET_ID => Next::LitFragment,
                    _ => Next::Length,
                };

                (token, PACKET_TYPE_BITS, next)
            }
            Next::Length => {
                let length_type = self.read_bits(LENGTH_TYPE_BITS);

                let (token, bits_read) = if length_type == 1 {
                    let t = self.read_length_subpackets();
                    let len = t.unwrap_len_subpackets();

                    self.stack.push(Remaining::Packets(len));

                    (t, LENGTH_TYPE_BITS + LEN_SUBPACKETS_BITS)
                } else {
                    let t = self.read_length_bitwise();
                    let len = t.unwrap_len_bitwise();

                    self.stack.push(Remaining::Bits(len));

                    (t, LENGTH_TYPE_BITS + LEN_BITWISE_BITS)
                };

                (token, bits_read, Next::Version)
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

                (token, LIT_FRAGMENT_BITS, next)
            }
            Next::None => {
                return None;
            }
        };

        self.state = next_state;

        Some((result, bits_read))
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

impl<I: Iterator<Item = u8>> Decoder<I> {
    /// Creates a new [`Decoder`] from a bit-by-bit iterable.
    pub fn from_bits<T: IntoIterator<IntoIter = I>>(bits: T) -> Self {
        Self {
            lexer: Lexer::from_bits(bits),
            stack: vec![],
        }
    }

    /// Consumes the decoder to evaluate the expression.
    pub fn decode(mut self) -> u64 {
        loop {
            if let Some(result) = self.decode_next_token() {
                return result;
            }
        }
    }

    fn decode_next_token(&mut self) -> Option<u64> {
        let (token, bits) = self.lexer.next_token().unwrap();
        let mut should_collapse = false;

        match token {
            Token::PacketType(type_id) => match type_id {
                LIT_PACKET_ID => self.stack.push(Packet::Lit(0)),
                SUM_PACKET_ID => self.stack.push(Packet::Sum(None, vec![])),
                PRODUCT_PACKET_ID => self.stack.push(Packet::Product(None, vec![])),
                MIN_PACKET_ID => self.stack.push(Packet::Min(None, vec![])),
                MAX_PACKET_ID => self.stack.push(Packet::Max(None, vec![])),
                GT_PACKET_ID => self.stack.push(Packet::Gt(None, vec![])),
                LT_PACKET_ID => self.stack.push(Packet::Lt(None, vec![])),
                EQ_PACKET_ID => self.stack.push(Packet::Eq(None, vec![])),
                _ => unreachable!(),
            },
            Token::LiteralFragment(is_cont, fragment) => {
                if let Some(Packet::Lit(v)) = self.stack.last_mut() {
                    *v = *v << 4 | fragment as u64 & 0xf;
                } else {
                    panic!("Receiver literal fragment in incorrect state");
                }

                if !is_cont {
                    should_collapse = true;
                }
            }
            Token::LengthBitwise(len) => {
                let rem = self.stack.last_mut().unwrap().remaining_as_mut();

                // workaround so that apply_read_bits calculates the correct value
                *rem = Some(Remaining::Bits(len + bits as u16));
            }
            Token::LengthSubpackets(len) => {
                let rem = self.stack.last_mut().unwrap().remaining_as_mut();

                *rem = Some(Remaining::Packets(len));
            }
            Token::Version(_) => (),
        };

        self.apply_read_bits(bits);

        if should_collapse {
            return self.collapse_stack();
        }

        None
    }

    fn apply_read_bits(&mut self, bits: usize) {
        for packet in &mut self.stack {
            if let Some(Remaining::Bits(n)) = packet.remaining_mut() {
                // *n -= bits as u16;
                *n = n
                    .checked_sub(bits as u16)
                    .expect("Underflow when calculating remaining bits in packet");
            }
        }
    }

    fn collapse_stack(&mut self) -> Option<u64> {
        use Packet::*;

        let mut value = None;
        loop {
            match self.stack.last_mut() {
                Some(Lit(v)) => {
                    value = Some(*v);
                }
                Some(Sum(rem, values)) => {
                    if let Some(val) = value.take() {
                        values.push(val);
                    }

                    match rem {
                        Some(Remaining::Packets(1) | Remaining::Bits(0)) => {
                            let sum = values.iter().sum();
                            value = Some(sum);
                        }
                        Some(Remaining::Packets(n)) => {
                            *n -= 1;
                            return None;
                        }
                        Some(Remaining::Bits(_)) => {
                            return None;
                        }
                        _ => unreachable!(),
                    }
                }
                Some(Product(rem, values)) => {
                    if let Some(val) = value.take() {
                        values.push(val);
                    }

                    match rem {
                        Some(Remaining::Packets(1) | Remaining::Bits(0)) => {
                            let product = values.iter().product();
                            value = Some(product);
                        }
                        Some(Remaining::Packets(n)) => {
                            *n -= 1;
                            return None;
                        }
                        Some(Remaining::Bits(_)) => {
                            return None;
                        }
                        _ => unreachable!(),
                    }
                }
                Some(Min(rem, values)) => {
                    if let Some(val) = value.take() {
                        values.push(val);
                    }

                    match rem {
                        Some(Remaining::Packets(1) | Remaining::Bits(0)) => {
                            let min = values.iter().copied().min().unwrap();
                            value = Some(min);
                        }
                        Some(Remaining::Packets(n)) => {
                            *n -= 1;
                            return None;
                        }
                        Some(Remaining::Bits(_)) => {
                            return None;
                        }
                        _ => unreachable!(),
                    }
                }
                Some(Max(rem, values)) => {
                    if let Some(val) = value.take() {
                        values.push(val);
                    }

                    match rem {
                        Some(Remaining::Packets(1) | Remaining::Bits(0)) => {
                            let max = values.iter().copied().max().unwrap();
                            value = Some(max);
                        }
                        Some(Remaining::Packets(n)) => {
                            *n -= 1;
                            return None;
                        }
                        Some(Remaining::Bits(_)) => {
                            return None;
                        }
                        _ => unreachable!(),
                    }
                }
                Some(Gt(rem, values)) => {
                    if let Some(val) = value.take() {
                        values.push(val);
                    }

                    match rem {
                        Some(Remaining::Packets(1) | Remaining::Bits(0)) => {
                            let result = match values.as_slice() {
                                [a, b] => (a > b) as u64,
                                _ => panic!("Incorrect number of operands for '>'"),
                            };

                            value = Some(result);
                        }
                        Some(Remaining::Packets(n)) => {
                            *n -= 1;
                            return None;
                        }
                        Some(Remaining::Bits(_)) => {
                            return None;
                        }
                        _ => unreachable!(),
                    }
                }
                Some(Lt(rem, values)) => {
                    if let Some(val) = value.take() {
                        values.push(val);
                    }

                    match rem {
                        Some(Remaining::Packets(1) | Remaining::Bits(0)) => {
                            let result = match values.as_slice() {
                                [a, b] => (a < b) as u64,
                                _ => panic!("Incorrect number of operands for '<'"),
                            };

                            value = Some(result);
                        }
                        Some(Remaining::Packets(n)) => {
                            *n -= 1;
                            return None;
                        }
                        Some(Remaining::Bits(_)) => {
                            return None;
                        }
                        _ => unreachable!(),
                    }
                }
                Some(Eq(rem, values)) => {
                    if let Some(val) = value.take() {
                        values.push(val);
                    }

                    match rem {
                        Some(Remaining::Packets(1) | Remaining::Bits(0)) => {
                            let result = match values.as_slice() {
                                [a, b] => (a == b) as u64,
                                _ => panic!("Incorrect number of operands for '=='"),
                            };

                            value = Some(result);
                        }
                        Some(Remaining::Packets(n)) => {
                            *n -= 1;
                            return None;
                        }
                        Some(Remaining::Bits(_)) => {
                            return None;
                        }
                        _ => unreachable!(),
                    }
                }
                None => {
                    assert!(value.is_some(), "Stack fully unwrapped without a value");

                    return value;
                }
            }

            self.stack.pop();
        }
    }
}

impl<I: Iterator<Item = u8>> Iterator for Lexer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().map(|(token, _)| token)
    }
}
