use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Vec<u32>> {
    let input = BufferedInput::parse_args("Day 9: Smoke Basin - Part 2")?;

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

fn sweep_basin_from(p: Point, floor: &mut Floor, visited: &mut HashSet<Point>) -> usize {
    let mut searchspace = vec![p];
    let mut result = 0;

    let dirs = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    while let Some((x, y)) = searchspace.pop() {
        result += 1;

        for neighbor in dirs.into_iter().map(|(dx, dy)| (x + dx, y + dy)) {
            if let Some(v) = floor.get(&neighbor) {
                if v != &9 && visited.insert(neighbor) {
                    searchspace.push(neighbor);
                }
            }
        }
    }

    result
}

fn search_basins(mut floor: Floor, width: i32, height: i32) -> Vec<usize> {
    let mut coords_to_check = (0..width).cartesian_product(0..height);
    let mut visited = HashSet::with_capacity(floor.len());
    let mut result = vec![];

    while let Some(next) = coords_to_check
        .by_ref()
        .find(|&p| floor[&p] != 9 && visited.insert(p))
    {
        let size = sweep_basin_from(next, &mut floor, &mut visited);
        result.push(size);
    }

    result
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let (floor, width, height) = init_floor(&input);

        let basins = search_basins(floor, width, height);

        basins
            .into_iter()
            .map(Reverse)
            .k_smallest(3)
            .map(|Reverse(x)| x)
            .product::<usize>()
    });
}
