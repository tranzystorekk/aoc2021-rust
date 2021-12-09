use std::collections::HashMap;
use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Vec<u32>> {
    let input = BufferedInput::parse_args("Day 9: Smoke Basin - Part 1")?;

    input
        .lines()
        .map_ok(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .try_collect()?
}

type Point = (i32, i32);
type Floor = HashMap<Point, u32>;

fn init_floor(rows: &[Vec<u32>]) -> (Floor, i32, i32) {
    let col_size = rows.len();
    let row_size = rows[0].len();

    let floor = rows
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.iter()
                .copied()
                .enumerate()
                .map(move |(x, v)| ((x as i32, y as i32), v))
        })
        .collect();

    (floor, row_size as i32, col_size as i32)
}

fn check_lowpoint((x, y): Point, floor: &Floor) -> Option<u32> {
    let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let v = floor[&(x, y)];

    dirs.into_iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter_map(|p| floor.get(&p))
        .all(|&val| val > v)
        .then(|| v + 1)
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let (floor, width, height) = init_floor(&input);
        let coords = (0..width).cartesian_product(0..height);

        coords
            .filter_map(|p| check_lowpoint(p, &floor))
            .sum::<u32>()
    });
}
