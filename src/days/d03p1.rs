use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<String> {
    let input = BufferedInput::parse_args("Day 3: Binary Diagnostic - Part 1")?;

    input.lines().try_collect()?
}

fn decode(occurs: &[usize], n: usize) -> (usize, usize) {
    let half = n / 2;
    let mut gamma = 0;
    let mut epsilon = 0;

    for (i, v) in occurs.iter().rev().enumerate() {
        if v > &half {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    (gamma, epsilon)
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut occurs = vec![0; input[0].len()];

        for (i, b) in input.iter().flat_map(|s| s.chars().enumerate()) {
            if b == '1' {
                occurs[i] += 1;
            }
        }

        let (gamma, epsilon) = decode(&occurs, input.len());

        gamma * epsilon
    });
}
