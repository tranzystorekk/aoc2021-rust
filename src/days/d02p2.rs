use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<Steer> {
    let input = BufferedInput::parse_args("Day 2: Dive! - Part 2")?;

    input
        .lines()
        .map_ok(|l| {
            let (cmd, n) = scan_fmt!(&l, "{} {d}", String, _).unwrap();

            match cmd.as_str() {
                "forward" => Steer::Forward(n),
                "down" => Steer::Down(n),
                "up" => Steer::Up(n),
                _ => unreachable!(),
            }
        })
        .try_collect()?
}

enum Steer {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let (hor, depth, _) = input
            .into_iter()
            .fold((0, 0, 0), |(hor, depth, aim), steer| match steer {
                Steer::Forward(n) => (hor + n, depth + aim * n, aim),
                Steer::Down(n) => (hor, depth, aim + n),
                Steer::Up(n) => (hor, depth, aim - n),
            });

        hor * depth
    });
}
