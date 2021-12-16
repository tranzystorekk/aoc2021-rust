//! The BITS decoder utilities

use std::collections::VecDeque;

pub enum Token {
    Version(u8),
    PacketTypeId(u8),
    LengthBits(u16),
    LengthSubpackets(u16),
    LiteralFragment(bool, u8),
}

struct Lexer<I> {
    it: I,
    buf: VecDeque<u8>,
    unread: usize,
}

pub struct Decoder {}

impl<I: Iterator<Item = u8>> Lexer<I> {
    fn from_hexes<T: IntoIterator<IntoIter = I>>(it: T) -> Self {
        Self {
            it: it.into_iter(),
            buf: [].into(),
            unread: 0,
        }
    }

    pub fn next(&mut self) -> Token {
        todo!()
    }

    // read at most `n` bits (up to 16) to `buf`
    // and return the number of bits actually read
    fn read_bits(&mut self, n: usize, buf: &mut u16) -> usize {
        assert!(n <= 16, "Attempted to read more than 16 bits at once");

        let missing = if n > self.unread { n - self.unread } else { 0 };
        let bytes_to_read = missing / 4 + if missing % 4 != 0 { 1 } else { 0 };

        self.buf.extend(self.it.by_ref().take(bytes_to_read));

        todo!()
    }
}
