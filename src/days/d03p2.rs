use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Vec<u8>> {
    let input = BufferedInput::parse_args("Day 3: Binary Diagnostic - Part 2")?;

    input.lines().map_ok(String::into_bytes).try_collect()?
}

fn split<'a>(values: &[&'a [u8]], pos: usize) -> (Vec<&'a [u8]>, Vec<&'a [u8]>) {
    let len = values.len();
    let half = len / 2;

    let n_ones = values.iter().filter(|val| val[pos] == b'1').count();

    if len % 2 == 0 && n_ones == half {
        return values.iter().partition(|v| v[pos] == b'1');
    }

    let most_common_bit = n_ones > half;

    values.iter().partition(|v| {
        let bit = v[pos] == b'1';

        bit == most_common_bit
    })
}

fn to_dec(bin: &[u8]) -> usize {
    bin.iter()
        .rev()
        .enumerate()
        .map(|(i, b)| ((b == &b'1') as usize) << i)
        .sum()
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;
    let len = input[0].len();

    aoc_utils::measure_and_print(|| {
        let mut most: Vec<_> = input.iter().map(Vec::as_slice).collect();
        let mut least: Vec<_> = input.iter().map(Vec::as_slice).collect();

        let (mut m_found, mut l_found) = (false, false);

        for i in 0..len {
            if !m_found {
                let (m, _) = split(&most, i);
                most = m;

                if most.len() == 1 {
                    m_found = true;
                }
            }
            if !l_found {
                let (_, l) = split(&least, i);
                least = l;

                if least.len() == 1 {
                    l_found = true;
                }
            }

            if m_found && l_found {
                break;
            }
        }

        let oxygen = to_dec(most[0]);
        let co2 = to_dec(least[0]);

        oxygen * co2
    });
}
