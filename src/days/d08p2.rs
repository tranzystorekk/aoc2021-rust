use std::{collections::HashSet, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Entry> {
    let input = BufferedInput::parse_args("Day 8: Seven Segment Search - Part 2")?;

    input
        .lines()
        .map_ok(|l| {
            let (pats, a, b, c, d) = scan_fmt!(
                &l,
                "{[a-g ]} | {} {} {} {}",
                String,
                String,
                String,
                String,
                String
            )
            .unwrap();

            let pats = pats
                .split_whitespace()
                .map(|p| p.chars().collect())
                .collect();

            Entry {
                pats,
                outs: [
                    a.chars().collect(),
                    b.chars().collect(),
                    c.chars().collect(),
                    d.chars().collect(),
                ],
            }
        })
        .try_collect()?
}

type Segs = HashSet<char>;

struct Entry {
    pats: Vec<Segs>,
    outs: [Segs; 4],
}

impl Entry {
    // segment numeration:
    //  _  <- 0
    // | | <- 1 2
    //  _  <- 3
    // | | <- 4 5
    //  _  <- 6
    fn decode(&self) -> u32 {
        let one = self.find_1();
        let four = self.find_4();
        let segs13: Segs = four.difference(one).copied().collect();

        let p235 = self.pats.iter().filter(|s| s.len() == 5).collect_vec();
        let p069 = self.pats.iter().filter(|s| s.len() == 6).collect_vec();

        let three = p235.iter().find(|p| p.is_superset(one)).unwrap();
        let five = p235.iter().find(|p| p.is_superset(&segs13)).unwrap();
        let nine = p069.iter().find(|p| p.is_superset(four)).unwrap();
        let six = p069
            .iter()
            .find(|p| p.is_superset(&segs13) && *p != nine)
            .unwrap();

        self.outs
            .iter()
            .map(|out| match out.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                5 => {
                    if &out == three {
                        3
                    } else if &out == five {
                        5
                    } else {
                        2
                    }
                }
                6 => {
                    if &out == nine {
                        9
                    } else if &out == six {
                        6
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            })
            .reduce(|acc, d| 10 * acc + d)
            .unwrap()
    }

    fn find_1(&self) -> &Segs {
        self.pats.iter().find(|s| s.len() == 2).unwrap()
    }

    fn find_4(&self) -> &Segs {
        self.pats.iter().find(|s| s.len() == 4).unwrap()
    }
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| input.iter().map(|entry| entry.decode()).sum::<u32>());
}
