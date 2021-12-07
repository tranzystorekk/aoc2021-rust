use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<i32> {
    let input = BufferedInput::parse_args("Day 7: The Treachery of Whales - Part 2")?;

    let line = input.unwrapped_lines().next().unwrap();

    line.split(',').map(|n| n.parse().unwrap()).collect()
}

fn gauss(v: i32) -> i32 {
    (v * (v + 1)) / 2
}

fn fuel_cost(p: i32, x: i32) -> i32 {
    let d = (p - x).abs();

    gauss(d)
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| -> i32 {
        let (min, max) = input.iter().copied().minmax().into_option().unwrap();

        (min..=max)
            .map(|p| input.iter().map(|&crab| fuel_cost(p, crab)).sum())
            .min()
            .unwrap()
    });
}
