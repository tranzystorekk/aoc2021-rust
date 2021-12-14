use std::collections::HashMap;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> (Template, PairInserts) {
    let input = BufferedInput::parse_args("Day 14: Extended Polymerization - Part 1")?;

    let mut lines = input.unwrapped_lines();

    let template = lines.next().unwrap().into_bytes();

    lines.next();

    let inserts = lines
        .map(|l| {
            let (pair, insert) = scan_fmt!(&l, "{} -> {}", String, char).unwrap();

            let pair = match *pair.as_bytes() {
                [a, b] => (a, b),
                _ => unreachable!(),
            };

            (pair, insert as u8)
        })
        .collect();

    (template, inserts)
}

type Template = Vec<u8>;
type PairInserts = HashMap<(u8, u8), u8>;

fn step_polymerize(template: Template, inserts: &PairInserts) -> Template {
    let inserts = template
        .iter()
        .tuple_windows()
        .map(|(&a, &b)| match inserts.get(&(a, b)) {
            Some(&el) => el,
            None => 0,
        })
        .collect_vec();

    template
        .into_iter()
        .interleave(inserts)
        .filter(|&el| el != 0)
        .collect()
}

#[anyhoo::anyhoo]
fn main() {
    let (template, inserts) = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let polymerized = (0..10).fold(template, |curr, _| step_polymerize(curr, &inserts));

        let counts = polymerized.into_iter().counts();

        let (min, max) = counts.into_values().minmax().into_option().unwrap();

        max - min
    });
}
