use std::collections::HashMap;
use std::io::BufRead;
use std::iter::repeat;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Line> {
    let input = BufferedInput::parse_args("Day 5: Hydrothermal Venture - Part 2")?;

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
    fn points(&self) -> Box<dyn Iterator<Item = (i32, i32)>> {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;

        if x1 == x2 {
            let start_y = std::cmp::min(y1, y2);
            let end_y = std::cmp::max(y1, y2);
            let it = repeat(x1).zip(start_y..=end_y);

            return Box::new(it);
        }

        if y1 == y2 {
            let start_x = std::cmp::min(x1, x2);
            let end_x = std::cmp::max(x1, x2);
            let it = (start_x..=end_x).zip(repeat(y1));

            return Box::new(it);
        }

        let it_x: Box<dyn Iterator<Item = i32>> = if x1 < x2 {
            Box::new(x1..=x2)
        } else {
            Box::new((x2..=x1).rev())
        };

        let it_y: Box<dyn Iterator<Item = i32>> = if y1 < y2 {
            Box::new(y1..=y2)
        } else {
            Box::new((y2..=y1).rev())
        };

        let it = (it_x).zip(it_y);

        Box::new(it)
    }
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut covers = HashMap::new();

        for point in input.into_iter().flat_map(|l| l.points()) {
            *covers.entry(point).or_default() += 1;
        }

        covers.into_values().filter(|&n: &usize| n > 1).count()
    });
}
