use std::io::BufRead;

use aoc_utils::BufferedInput;
use itertools::Itertools;

#[anyhoo::anyhoo]
fn parse_input() -> Vec<String> {
    let input = BufferedInput::parse_args("Day 10: Syntax Scoring - Part 2")?;

    input.lines().try_collect()?
}

fn check_incomplete(line: &str) -> Option<u64> {
    let result = line.chars().try_fold(vec![], |mut stack, c| {
        match (c, stack.last()) {
            ('(' | '[' | '{' | '<', _) => {
                stack.push(c);
            }
            (')', Some(&'(')) | (']', Some(&'[')) | ('}', Some(&'{')) | ('>', Some(&'<')) => {
                stack.pop();
            }
            (')' | ']' | '}' | '>', _) => {
                return None;
            }
            _ => unreachable!(),
        };

        Some(stack)
    });

    result.map(|completion| {
        completion.into_iter().rev().fold(0, |score, c| {
            let v = match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            };

            5 * score + v
        })
    })
}

#[anyhoo::anyhoo]
fn main() {
    let input = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let scores = input
            .into_iter()
            .filter_map(|l| check_incomplete(&l))
            .sorted_unstable()
            .collect_vec();

        scores[scores.len() / 2]
    });
}
