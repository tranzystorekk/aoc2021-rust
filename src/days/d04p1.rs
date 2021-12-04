use aoc_utils::BufferedInput;
use itertools::Itertools;
use ndarray::Array2;

#[anyhoo::anyhoo]
fn parse_input() -> (Vec<i32>, Vec<Board>) {
    let input = BufferedInput::parse_args("Day 4: Giant Squid - Part 1")?;

    let mut lines = input.unwrapped_lines();
    let draws = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let boards = lines.filter(|l| !l.is_empty()).chunks(5);
    let boards = boards
        .into_iter()
        .map(|rows| {
            let v = rows
                .flat_map(|r| {
                    r.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect_vec()
                })
                .map(|n| (n, false))
                .collect();

            Board::from_shape_vec((5, 5), v).unwrap()
        })
        .collect();

    (draws, boards)
}

type Board = Array2<(i32, bool)>;

fn mark_board(board: &mut Board, draw: i32) -> Option<i32> {
    for (n, marked) in board.iter_mut() {
        if n == &draw {
            *marked = true;
        }
    }

    let row_found = || {
        board
            .rows()
            .into_iter()
            .any(|r| r.iter().all(|(_, marked)| *marked))
    };
    let col_found = || {
        board
            .columns()
            .into_iter()
            .any(|c| c.iter().all(|(_, marked)| *marked))
    };

    if !row_found() && !col_found() {
        return None;
    }

    let lane_sum: i32 = board
        .iter()
        .filter(|(_, marked)| !marked)
        .map(|(n, _)| *n)
        .sum();
    let score = lane_sum * draw;

    Some(score)
}

#[anyhoo::anyhoo]
fn main() {
    let (draws, mut boards) = parse_input()?;

    aoc_utils::measure_and_print(|| {
        draws
            .into_iter()
            .find_map(|d| boards.iter_mut().find_map(|b| mark_board(b, d)))
            .unwrap()
    });
}
