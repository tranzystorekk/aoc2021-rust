use std::{collections::HashMap, io::BufRead};

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Line> {
    let input = BufferedInput::parse_args("Day 5: Hydrothermal Venture - Part 1")?;

    input
        .lines()
        .map_ok(|l| {
            let (x1, y1, x2, y2) = scan_fmt!(&l, "{d},{d} -> {d},{d}", _, _, _, _).unwrap();

            Line {
                start: (x1, y1),
                end: (x2, y2),
            }
        })
        .try_collect()?
}

struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

impl Line {
    fn is_straight(&self) -> bool {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;

        x1 == x2 || y1 == y2
    }

    fn points(&self) -> impl Iterator<Item = (i32, i32)> {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;

        if x1 == x2 {
            let start = std::cmp::min(y1, y2);
            let end = std::cmp::max(y1, y2);

            let f = move |y| (x1, y);
            let f: Box<dyn Fn(i32) -> (i32, i32)> = Box::new(f);

            (start..=end).map(f)
        } else if y1 == y2 {
            let start = std::cmp::min(x1, x2);
            let end = std::cmp::max(x1, x2);

            let f = move |x| (x, y1);
            let f: Box<dyn Fn(i32) -> (i32, i32)> = Box::new(f);

            (start..=end).map(f)
        } else {
            unreachable!()
        }
    }
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut covers = HashMap::new();
        let points = input
            .into_iter()
            .filter(|l| l.is_straight())
            .flat_map(|l| l.points());

        for point in points {
            *covers.entry(point).or_default() += 1;
        }

        covers.into_values().filter(|&n: &usize| n > 1).count()
    });
}
