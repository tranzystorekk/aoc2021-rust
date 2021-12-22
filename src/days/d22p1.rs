use std::collections::HashMap;
use std::io::BufRead;
use std::ops::RangeInclusive;

use aoc_utils::BufferedInput;
use itertools::{iproduct, Itertools};
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Step> {
    let input = BufferedInput::parse_args("Day 22: Reactor Reboot - Part 1")?;

    input
        .lines()
        .map_ok(|l| {
            let (state, x1, x2, y1, y2, z1, z2) = scan_fmt!(
                &l,
                "{} x={d}..{d},y={d}..{d},z={d}..{d}",
                String,
                _,
                _,
                _,
                _,
                _,
                _
            )
            .unwrap();

            match state.as_str() {
                "on" => Step::On(x1..=x2, y1..=y2, z1..=z2),
                "off" => Step::Off(x1..=x2, y1..=y2, z1..=z2),
                _ => unreachable!(),
            }
        })
        .try_collect()?
}

type Range = RangeInclusive<i32>;
type Grid = HashMap<(i32, i32, i32), bool>;

#[derive(Clone)]
enum Step {
    On(Range, Range, Range),
    Off(Range, Range, Range),
}

fn init_region_bounded(range: Range) -> Range {
    let (&start, &end) = (range.start(), range.end());

    let new_start = std::cmp::max(-50, start);
    let new_end = std::cmp::min(50, end);

    new_start..=new_end
}

fn execute_step(step: Step, grid: &mut Grid) {
    let (new_state, xs, ys, zs) = match step {
        Step::On(xs, ys, zs) => (true, xs, ys, zs),
        Step::Off(xs, ys, zs) => (false, xs, ys, zs),
    };

    for pos in iproduct!(
        init_region_bounded(xs),
        init_region_bounded(ys),
        init_region_bounded(zs)
    ) {
        *grid.entry(pos).or_default() = new_state;
    }
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let mut grid = HashMap::new();

        for step in input {
            execute_step(step, &mut grid);
        }

        grid.into_values().filter(|&state| state).count()
    });
}
