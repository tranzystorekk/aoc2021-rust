use aoc::bits::{hexes_to_bits, Lexer, Token};
use aoc_utils::BufferedInput;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<u8> {
    let input = BufferedInput::parse_args("Day 16: Packet Decoder - Part 1")?;

    let line = input.unwrapped_lines().next().unwrap();

    line.chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect()
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let bits = hexes_to_bits(input);
        let lexer = Lexer::from_bits(bits);

        lexer
            .filter_map(|token| match token {
                Token::Version(ver) => Some(ver as u64),
                _ => None,
            })
            .sum::<u64>()
    });
}
