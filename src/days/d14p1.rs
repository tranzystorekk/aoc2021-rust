use std::collections::HashMap;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> (String, PairInserts) {
    let input = BufferedInput::parse_args("Day 14: Extended Polymerization - Part 1")?;

    let mut lines = input.unwrapped_lines();

    let template = lines.next().unwrap();

    lines.next();

    let inserts = lines
        .map(|l| {
            let (pair, insert) = scan_fmt!(&l, "{} -> {}", String, char).unwrap();

            let pair = pair.chars().collect_tuple().unwrap();

            (pair, insert)
        })
        .collect();

    (template, inserts)
}

type Pairs = HashMap<(char, char), usize>;
type Counts = HashMap<char, usize>;
type PairInserts = HashMap<(char, char), char>;

fn init_auxiliary(sequence: &str) -> (Pairs, Counts) {
    let pairs = sequence.chars().tuple_windows().counts();
    let counts = sequence.chars().counts();

    (pairs, counts)
}

fn step_polymerize(pairs: Pairs, counts: &mut Counts, inserts: &PairInserts) -> Pairs {
    pairs
        .into_iter()
        .flat_map(|(pair @ (a, b), count)| match inserts.get(&pair) {
            Some(&el) => {
                *counts.entry(el).or_default() += count;

                vec![((a, el), count), ((el, b), count)]
            }
            None => vec![(pair, count)],
        })
        .into_grouping_map()
        .sum()
}

#[anyhoo::anyhoo]
fn main() {
    let (template, inserts) = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let (pairs, mut counts) = init_auxiliary(&template);

        (0..10).fold(pairs, |curr, _| {
            step_polymerize(curr, &mut counts, &inserts)
        });

        let (min, max) = counts.into_values().minmax().into_option().unwrap();

        max - min
    });
}
