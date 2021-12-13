use std::collections::HashSet;

use aoc_utils::BufferedInput;
use itertools::{chain, Itertools};
use scan_fmt::scan_fmt;

#[anyhoo::anyhoo]
fn parse_input() -> (Foil, Vec<Fold>) {
    let input = BufferedInput::parse_args("Day 13: Transparent Origami - Part 2")?;

    let mut lines = input.unwrapped_lines();

    let points = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| scan_fmt!(&l, "{d},{d}", _, _).unwrap())
        .collect();

    let folds = lines
        .map(|l| {
            let (axis, val) = scan_fmt!(&l, "fold along {}={d}", String, _).unwrap();

            match axis.as_str() {
                "x" => Fold::Vertical(val),
                "y" => Fold::Horizontal(val),
                _ => unreachable!(),
            }
        })
        .collect();

    (points, folds)
}

type Point = (i32, i32);
type Foil = HashSet<Point>;

#[derive(Clone, Copy, Debug)]
enum Fold {
    Horizontal(i32),
    Vertical(i32),
}

fn make_fold(foil: Foil, fold: Fold) -> Foil {
    match fold {
        Fold::Horizontal(y) => fold_horizontal(foil, y),
        Fold::Vertical(x) => fold_vertical(foil, x),
    }
}

fn fold_vertical(foil: Foil, fold_x: i32) -> Foil {
    let (folded, original): (Vec<_>, Vec<_>) = foil.iter().partition(|(x, _)| x > &fold_x);

    let folded = folded.into_iter().map(|(x, y)| {
        let dist = x - fold_x;
        let new_x = fold_x - dist;

        (new_x, y)
    });

    chain!(original, folded).collect()
}

fn fold_horizontal(foil: Foil, fold_y: i32) -> Foil {
    let (folded, original): (Vec<_>, Vec<_>) = foil.iter().partition(|(_, y)| y > &fold_y);

    let folded = folded.into_iter().map(|(x, y)| {
        let dist = y - fold_y;
        let new_y = fold_y - dist;

        (x, new_y)
    });

    chain!(original, folded).collect()
}

fn print_foil(foil: &Foil) -> String {
    let (width, height) = foil.iter().fold((0, 0), |(w, h), &(x, y)| {
        (std::cmp::max(w, x), std::cmp::max(h, y))
    });

    (0..=height)
        .map(|y| {
            (0..=width)
                .map(move |x| if foil.contains(&(x, y)) { '#' } else { ' ' })
                .collect::<String>()
        })
        .join("\n")
}

#[anyhoo::anyhoo]
fn main() {
    let (foil, folds) = parse_input()?;

    aoc_utils::measure_and_print(|| {
        let folded = folds.into_iter().fold(foil, make_fold);

        print_foil(&folded)
    });
}
