use std::collections::HashMap;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Vec<u32>> {
    let input = BufferedInput::parse_args("Day 11: Dumbo Octopus - Part 2")?;

    input
        .unwrapped_lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

type Point = (i32, i32);
type Cavern = HashMap<Point, u32>;

fn init_cavern(rows: &[Vec<u32>]) -> Cavern {
    rows.iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .copied()
                .enumerate()
                .map(move |(x, v)| ((x as i32, y as i32), v))
        })
        .collect()
}

fn step_flash(cavern: &mut Cavern) -> usize {
    let mut flashed = vec![];

    for (pos, energy) in cavern.iter_mut() {
        *energy += 1;

        if *energy > 9 {
            flashed.push(*pos);
        }
    }

    let mut searchspace = flashed.clone();

    while let Some((x, y)) = searchspace.pop() {
        let dirs = (-1..=1).cartesian_product(-1..=1).filter(|p| p != &(0, 0));

        for neighbor in dirs.map(|(dx, dy)| (x + dx, y + dy)) {
            if let Some(v) = cavern.get_mut(&neighbor) {
                *v += 1;

                if *v == 10 {
                    flashed.push(neighbor);
                    searchspace.push(neighbor);
                }
            }
        }
    }

    for &pos in &flashed {
        cavern.entry(pos).and_modify(|energy| *energy = 0);
    }

    flashed.len()
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut cavern = init_cavern(&input);

        let step = std::iter::repeat_with(|| step_flash(&mut cavern))
            .position(|n| n == 100)
            .unwrap();

        step + 1
    });
}
