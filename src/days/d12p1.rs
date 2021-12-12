use std::collections::{HashMap, HashSet};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Tunnels {
    let input = BufferedInput::parse_args("Day 12: Passage Pathing - Part 1")?;

    input
        .unwrapped_lines()
        .flat_map(|l| {
            let (a, b) = scan_fmt!(&l, "{}-{}", String, String).unwrap();

            if a == "start" {
                return vec![(a, b)];
            }

            if b == "start" {
                return vec![(b, a)];
            }

            vec![(a.clone(), b.clone()), (b, a)]
        })
        .into_group_map()
}

type Tunnels = HashMap<String, Vec<String>>;

fn is_small(cave: &str) -> bool {
    cave.chars().all(|c| c.is_ascii_lowercase())
}

fn sweep_paths(tunnels: &Tunnels) -> usize {
    let mut searchspace: Vec<(&str, HashSet<&str>)> = vec![("start", ["start"].into())];
    let mut result = 0;

    while let Some((curr, small_visited)) = searchspace.pop() {
        if curr == "end" {
            result += 1;
            continue;
        }

        for neighbor in &tunnels[curr] {
            let mut new_visited = small_visited.clone();
            if !is_small(neighbor) {
                searchspace.push((neighbor, new_visited));
                continue;
            }

            if new_visited.insert(neighbor) {
                searchspace.push((neighbor, new_visited));
            }
        }
    }

    result
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| sweep_paths(&input));
}
