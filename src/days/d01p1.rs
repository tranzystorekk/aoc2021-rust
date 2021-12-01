use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<i32> {
    let input = BufferedInput::parse_args("Day 1: Sonar Sweep - Part 1")?;

    input.lines().map_ok(|l| l.parse().unwrap()).try_collect()?
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        input
            .into_iter()
            .tuple_windows()
            .filter(|(prev, curr)| prev < curr)
            .count()
    });
}
