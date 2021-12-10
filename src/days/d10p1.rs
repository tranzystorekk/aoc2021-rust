use std::{io::BufRead, ops::ControlFlow};

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<String> {
    let input = BufferedInput::parse_args("Day 10: Syntax Scoring - Part 1")?;

    input.lines().try_collect()?
}

fn check_corrupted(line: &str) -> Option<u64> {
    let result = line.chars().try_fold(vec![], |mut stack, c| {
        match (c, stack.last()) {
            (open @ ('(' | '[' | '{' | '<'), _) => {
                stack.push(open);
            }
            (')', Some(&'(')) | (']', Some(&'[')) | ('}', Some(&'{')) | ('>', Some(&'<')) => {
                stack.pop();
            }
            (')', _) => {
                return ControlFlow::Break(3);
            }
            (']', _) => {
                return ControlFlow::Break(57);
            }
            ('}', _) => {
                return ControlFlow::Break(1197);
            }
            ('>', _) => {
                return ControlFlow::Break(25137);
            }
            _ => unreachable!(),
        };

        ControlFlow::Continue(stack)
    });

    match result {
        ControlFlow::Break(score) => Some(score),
        _ => None,
    }
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        input
            .into_iter()
            .filter_map(|l| check_corrupted(&l))
            .sum::<u64>()
    });
}
