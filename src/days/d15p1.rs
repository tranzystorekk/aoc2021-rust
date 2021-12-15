use std::collections::{BinaryHeap, HashMap};

use aoc_utils::BufferedInput;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Vec<u32>> {
    let input = BufferedInput::parse_args("Day 15: Chiton - Part 1")?;

    input
        .unwrapped_lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

type Point = (i32, i32);
type Map = HashMap<Point, u32>;

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    pos: Point,
    risk: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn init_map(rows: Vec<Vec<u32>>) -> Map {
    rows.into_iter()
        .enumerate()
        .flat_map(|(y, r)| {
            r.into_iter()
                .enumerate()
                .map(move |(x, v)| ((x as i32, y as i32), v))
        })
        .collect()
}

fn sweep_risk(map: &Map, size: i32) -> u32 {
    let mut distances = HashMap::from([((0, 0), 0)]);
    let mut heap = BinaryHeap::from([State {
        pos: (0, 0),
        risk: 0,
    }]);

    let end = (size - 1, size - 1);
    let adjacent = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(State {
        pos: pos @ (x, y),
        risk: cur_risk,
    }) = heap.pop()
    {
        if pos == end {
            return cur_risk;
        }

        let lowest_risk = distances.entry(pos).or_insert(u32::MAX);
        if &cur_risk > lowest_risk {
            continue;
        }

        let neighbors = adjacent.into_iter().map(|(dx, dy)| (x + dx, y + dy));

        for neighbor in neighbors {
            if let Some(&next_risk) = map.get(&neighbor) {
                let next = State {
                    pos: neighbor,
                    risk: cur_risk + next_risk,
                };
                let neighbor_risk = distances.entry(neighbor).or_insert(u32::MAX);

                if &next.risk < neighbor_risk {
                    *neighbor_risk = next.risk;
                    heap.push(next);
                }
            }
        }
    }

    unreachable!()
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let size = input.len() as i32;
        let map = init_map(input);

        sweep_risk(&map, size)
    });
}
